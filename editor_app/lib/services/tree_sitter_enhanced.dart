import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'package:flutter/material.dart';
import 'tree_sitter/tree_sitter_bindings.dart';
import 'tree_sitter/query_engine.dart';
import '../models/token.dart';

/// 增强的 Tree-sitter 服务
/// 集成高亮、符号导航、代码折叠等功能
class TreeSitterEnhanced {
  static TreeSitterEnhanced? _instance;
  static TreeSitterEnhanced get instance =>
      _instance ??= TreeSitterEnhanced._();

  TreeSitterEnhanced._();

  // Tree-sitter 相关
  DynamicLibrary? _treeSitterLib;
  DynamicLibrary? _pythonLib;
  TreeSitter? _bindings;
  Pointer<TSLanguage>? _pythonLanguage;
  TreeSitterQueryEngine? _queryEngine;

  // 缓存
  Map<String, List<Token>> _tokenCache = {};
  Map<String, List<Symbol>> _symbolCache = {};
  Map<String, List<Fold>> _foldCache = {};
  bool _isInitialized = false;

  // VS Code 风格的颜色方案
  static const Map<TokenKind, TextStyle> _tokenStyles = {
    TokenKind.keyword: TextStyle(
      color: Color(0xFF569CD6), // 蓝色 - 关键字
      fontWeight: FontWeight.w500,
    ),
    TokenKind.identifier: TextStyle(
      color: Color(0xFF9CDCFE), // 浅蓝色 - 标识符
    ),
    TokenKind.string: TextStyle(
      color: Color(0xFFCE9178), // 橙色 - 字符串
    ),
    TokenKind.number: TextStyle(
      color: Color(0xFFB5CEA8), // 绿色 - 数字
    ),
    TokenKind.comment: TextStyle(
      color: Color(0xFF6A9955), // 绿色 - 注释
      fontStyle: FontStyle.italic,
    ),
    TokenKind.punctuation: TextStyle(
      color: Color(0xFFD4D4D4), // 白色 - 标点符号
    ),
    TokenKind.operator: TextStyle(
      color: Color(0xFFD4D4D4), // 白色 - 操作符
    ),
    TokenKind.whitespace: TextStyle(
      color: Color(0xFFD4D4D4), // 默认颜色 - 空白字符
    ),
    TokenKind.unknown: TextStyle(
      color: Color(0xFFD4D4D4), // 默认颜色 - 未知字符
    ),
  };

  // 默认文本样式
  static const TextStyle _defaultStyle = TextStyle(
    color: Color(0xFFD4D4D4), // VS Code 默认前景色
    fontSize: 14,
    fontFamily: 'JetBrains Mono',
    height: 1.3,
  );

  /// 初始化 Tree-sitter
  Future<void> initialize() async {
    if (_isInitialized) return;

    try {
      // 尝试多个可能的路径
      final exeDir = File(Platform.resolvedExecutable).parent.path;
      final currentDir = Directory.current.path;

      // 尝试多个可能的路径
      final possiblePaths = [
        '$exeDir/libtree-sitter.dylib',
        '$currentDir/macos/Runner/libtree-sitter.dylib',
        'macos/Runner/libtree-sitter.dylib',
        'libtree-sitter.dylib', // 当前目录
      ];

      String? treeSitterPath;
      String? pythonPath;

      for (final basePath in possiblePaths) {
        final treeSitterFile = File(basePath);
        final pythonFile = File(basePath.replaceAll(
            'libtree-sitter.dylib', 'libtree_sitter_python.dylib'));

        print('🔍 检查路径: $basePath (存在: ${treeSitterFile.existsSync()})');
        print('🔍 检查路径: ${pythonFile.path} (存在: ${pythonFile.existsSync()})');

        if (treeSitterFile.existsSync() && pythonFile.existsSync()) {
          treeSitterPath = basePath;
          pythonPath = basePath.replaceAll(
              'libtree-sitter.dylib', 'libtree_sitter_python.dylib');
          print('✅ 找到文件: $treeSitterPath');
          break;
        }
      }

      if (treeSitterPath == null || pythonPath == null) {
        throw Exception('找不到 Tree-sitter 动态库文件');
      }

      print('🔍 尝试加载动态库: $treeSitterPath');
      _treeSitterLib = DynamicLibrary.open(treeSitterPath);
      print('🔍 尝试加载 Python 库: $pythonPath');
      _pythonLib = DynamicLibrary.open(pythonPath);

      // 创建绑定
      _bindings = TreeSitter(_treeSitterLib!);

      // 获取 Python 语言
      _pythonLanguage = _pythonLib!
          .lookup<NativeFunction<Pointer<Void> Function()>>(
              'tree_sitter_python')
          .asFunction<Pointer<Void> Function()>()()
          .cast();

      // 创建查询引擎
      _queryEngine = TreeSitterQueryEngine(_bindings!, _treeSitterLib!);

      _isInitialized = true;
      print('✅ Tree-sitter Enhanced 初始化成功');
    } catch (e) {
      print('❌ Tree-sitter Enhanced 初始化失败: $e');
      _isInitialized = false;
      return; // 失败时直接返回，不继续执行
    }
  }

  /// 语法高亮 - 返回 Token 列表
  List<Token> highlight(String text) {
    if (!_isInitialized) {
      return _fallbackTokenize(text);
    }

    // 检查缓存
    if (_tokenCache.containsKey(text)) {
      return _tokenCache[text]!;
    }

    try {
      // 创建解析器
      final parser = _bindings!.ts_parser_new();
      final success =
          _bindings!.ts_parser_set_language(parser, _pythonLanguage!);

      if (!success) {
        throw Exception('设置 Python 语言失败');
      }

      // 解析代码
      final codePtr = text.toNativeUtf8();
      final tree = _bindings!.ts_parser_parse_string(
        parser,
        nullptr,
        codePtr.cast(),
        text.length,
      );
      calloc.free(codePtr);

      if (tree.address == 0) {
        throw Exception('代码解析失败');
      }

      // 获取语法高亮 token
      final tokens = _getHighlightTokens(tree, text);

      // 清理资源
      _bindings!.ts_tree_delete(tree);
      _bindings!.ts_parser_delete(parser);

      // 缓存结果
      _tokenCache[text] = tokens;

      return tokens;
    } catch (e) {
      print('⚠️  Tree-sitter Enhanced highlight 失败，回退到基础 tokenizer: $e');
      return _fallbackTokenize(text);
    }
  }

  /// 构建 TextSpan - 直接用于 Flutter 渲染
  TextSpan buildTextSpan(String text, {TextStyle? baseStyle}) {
    final tokens = highlight(text);
    final spans = <TextSpan>[];
    int currentPosition = 0;

    for (final token in tokens) {
      // 处理 token 之前的未匹配文本
      if (token.start > currentPosition) {
        final unmatchedText = text.substring(currentPosition, token.start);
        spans.add(TextSpan(
          text: unmatchedText,
          style: baseStyle ?? _defaultStyle,
        ));
      }

      // 对于空格token，强制使用等宽字体
      if (token.kind == TokenKind.whitespace) {
        spans.add(TextSpan(
          text: token.text,
          style: (baseStyle ?? _defaultStyle).copyWith(
            fontFamily: 'JetBrains Mono',
            letterSpacing: 0.0, // 确保字符间距正常
          ),
        ));
      } else {
        // 添加当前 token 的样式化文本
        final tokenStyle = _tokenStyles[token.kind] ?? const TextStyle();
        spans.add(TextSpan(
          text: token.text,
          style: (baseStyle ?? _defaultStyle).merge(tokenStyle),
        ));
      }

      currentPosition = token.end;
    }

    // 处理剩余的未匹配文本
    if (currentPosition < text.length) {
      final remainingText = text.substring(currentPosition);
      spans.add(TextSpan(
        text: remainingText,
        style: baseStyle ?? _defaultStyle,
      ));
    }

    return TextSpan(
      style: baseStyle ?? _defaultStyle,
      children: spans,
    );
  }

  /// 符号导航 - 返回符号列表
  List<Symbol> getSymbols(String text) {
    if (!_isInitialized) {
      return [];
    }

    // 检查缓存
    if (_symbolCache.containsKey(text)) {
      return _symbolCache[text]!;
    }

    try {
      // 创建解析器
      final parser = _bindings!.ts_parser_new();
      final success =
          _bindings!.ts_parser_set_language(parser, _pythonLanguage!);

      if (!success) {
        throw Exception('设置 Python 语言失败');
      }

      // 解析代码
      final codePtr = text.toNativeUtf8();
      final tree = _bindings!.ts_parser_parse_string(
        parser,
        nullptr,
        codePtr.cast(),
        text.length,
      );
      calloc.free(codePtr);

      if (tree.address == 0) {
        throw Exception('代码解析失败');
      }

      // 获取符号
      final symbols = _getSymbolsFromTree(tree, text);

      // 清理资源
      _bindings!.ts_tree_delete(tree);
      _bindings!.ts_parser_delete(parser);

      // 缓存结果
      _symbolCache[text] = symbols;

      return symbols;
    } catch (e) {
      print('⚠️  Tree-sitter Enhanced getSymbols 失败: $e');
      return [];
    }
  }

  /// 代码折叠 - 返回可折叠区域列表
  List<Fold> getFolds(String text) {
    if (!_isInitialized) {
      return [];
    }

    // 检查缓存
    if (_foldCache.containsKey(text)) {
      return _foldCache[text]!;
    }

    try {
      // 创建解析器
      final parser = _bindings!.ts_parser_new();
      final success =
          _bindings!.ts_parser_set_language(parser, _pythonLanguage!);

      if (!success) {
        throw Exception('设置 Python 语言失败');
      }

      // 解析代码
      final codePtr = text.toNativeUtf8();
      final tree = _bindings!.ts_parser_parse_string(
        parser,
        nullptr,
        codePtr.cast(),
        text.length,
      );
      calloc.free(codePtr);

      if (tree.address == 0) {
        throw Exception('代码解析失败');
      }

      // 获取折叠区域
      final folds = _getFoldsFromTree(tree, text);

      // 清理资源
      _bindings!.ts_tree_delete(tree);
      _bindings!.ts_parser_delete(parser);

      // 缓存结果
      _foldCache[text] = folds;

      return folds;
    } catch (e) {
      print('⚠️  Tree-sitter Enhanced getFolds 失败: $e');
      return [];
    }
  }

  /// 从语法树获取高亮 token
  List<Token> _getHighlightTokens(Pointer<TSTree> tree, String text) {
    final tokens = <Token>[];

    try {
      // 获取语法高亮 token
      final exeDir = File(Platform.resolvedExecutable).parent.path;
      final appBundlePath = '$exeDir';
      final highlightsQuery = _queryEngine!
          .loadQuery('$appBundlePath/highlights.scm', _pythonLanguage!);
      final highlights =
          _queryEngine!.executeHighlights(highlightsQuery, tree, text);

      // 转换为 Token 对象
      for (final highlight in highlights) {
        tokens.add(Token(
          kind: _mapCaptureToTokenKind(highlight.captureName),
          start: highlight.start,
          end: highlight.end,
          text: text.substring(highlight.start, highlight.end),
        ));
      }

      // 按位置排序
      tokens.sort((a, b) => a.start.compareTo(b.start));

      // 填充空白字符
      return _fillWhitespaceTokens(tokens, text);
    } catch (e) {
      print('⚠️  获取语法高亮 token 失败: $e');
      return _fallbackTokenize(text);
    }
  }

  /// 从语法树获取符号
  List<Symbol> _getSymbolsFromTree(Pointer<TSTree> tree, String text) {
    try {
      // 获取符号
      final exeDir = File(Platform.resolvedExecutable).parent.path;
      final appBundlePath = '$exeDir';
      final localsQuery = _queryEngine!
          .loadQuery('$appBundlePath/locals.scm', _pythonLanguage!);
      return _queryEngine!.executeLocals(localsQuery, tree, text);
    } catch (e) {
      print('⚠️  获取符号失败: $e');
      return [];
    }
  }

  /// 从语法树获取折叠区域
  List<Fold> _getFoldsFromTree(Pointer<TSTree> tree, String text) {
    try {
      // 获取折叠区域
      final exeDir = File(Platform.resolvedExecutable).parent.path;
      final appBundlePath = '$exeDir';
      final indentsQuery = _queryEngine!
          .loadQuery('$appBundlePath/indents.scm', _pythonLanguage!);
      return _queryEngine!.executeFolds(indentsQuery, tree, text);
    } catch (e) {
      print('⚠️  获取折叠区域失败: $e');
      return [];
    }
  }

  /// 将捕获名称映射到 TokenKind
  TokenKind _mapCaptureToTokenKind(String captureName) {
    switch (captureName) {
      case 'keyword':
        return TokenKind.keyword;
      case 'string':
        return TokenKind.string;
      case 'number':
        return TokenKind.number;
      case 'comment':
        return TokenKind.comment;
      case 'function':
      case 'function.builtin':
      case 'function.method':
        return TokenKind.identifier;
      case 'variable':
      case 'parameter':
      case 'field':
        return TokenKind.identifier;
      case 'type':
        return TokenKind.identifier;
      case 'operator':
        return TokenKind.operator;
      case 'punctuation':
      case 'punctuation.special':
        return TokenKind.punctuation;
      default:
        return TokenKind.unknown;
    }
  }

  /// 填充空白字符 token
  List<Token> _fillWhitespaceTokens(List<Token> tokens, String text) {
    final result = <Token>[];
    int currentPos = 0;

    for (final token in tokens) {
      // 添加 token 之前的空白字符
      if (token.start > currentPos) {
        final whitespace = text.substring(currentPos, token.start);
        if (whitespace.isNotEmpty) {
          result.add(Token(
            kind: TokenKind.whitespace,
            start: currentPos,
            end: token.start,
            text: whitespace,
          ));
        }
      }

      result.add(token);
      currentPos = token.end;
    }

    // 添加剩余的空白字符
    if (currentPos < text.length) {
      final remaining = text.substring(currentPos);
      if (remaining.isNotEmpty) {
        result.add(Token(
          kind: TokenKind.whitespace,
          start: currentPos,
          end: text.length,
          text: remaining,
        ));
      }
    }

    return result;
  }

  /// 回退到基础 tokenizer
  List<Token> _fallbackTokenize(String text) {
    final tokens = <Token>[];
    int position = 0;

    while (position < text.length) {
      final char = text[position];

      // 跳过空白字符
      if (_isWhitespace(char)) {
        final start = position;
        while (position < text.length && _isWhitespace(text[position])) {
          position++;
        }
        tokens.add(Token(
          kind: TokenKind.whitespace,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 处理注释
      if (char == '#') {
        final start = position;
        while (position < text.length && text[position] != '\n') {
          position++;
        }
        tokens.add(Token(
          kind: TokenKind.comment,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 处理字符串
      if (char == '"' || char == "'") {
        final start = position;
        final quote = char;
        position++;

        while (position < text.length) {
          if (text[position] == quote) {
            position++;
            break;
          }
          if (text[position] == '\\' && position + 1 < text.length) {
            position += 2;
          } else {
            position++;
          }
        }

        tokens.add(Token(
          kind: TokenKind.string,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 处理数字
      if (_isDigit(char)) {
        final start = position;
        while (position < text.length &&
            (_isDigit(text[position]) || text[position] == '.')) {
          position++;
        }
        tokens.add(Token(
          kind: TokenKind.number,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 处理标识符和关键字
      if (_isIdentifierStart(char)) {
        final start = position;
        while (position < text.length && _isIdentifierPart(text[position])) {
          position++;
        }
        final identifier = text.substring(start, position);

        tokens.add(Token(
          kind:
              _isKeyword(identifier) ? TokenKind.keyword : TokenKind.identifier,
          start: start,
          end: position,
          text: identifier,
        ));
        continue;
      }

      // 处理操作符和标点符号
      if (_isOperator(char)) {
        tokens.add(Token(
          kind: TokenKind.operator,
          start: position,
          end: position + 1,
          text: char,
        ));
      } else if (_isPunctuation(char)) {
        tokens.add(Token(
          kind: TokenKind.punctuation,
          start: position,
          end: position + 1,
          text: char,
        ));
      } else {
        tokens.add(Token(
          kind: TokenKind.unknown,
          start: position,
          end: position + 1,
          text: char,
        ));
      }
      position++;
    }

    return tokens;
  }

  // 辅助方法
  bool _isKeyword(String text) {
    const keywords = {
      'def',
      'class',
      'if',
      'else',
      'elif',
      'for',
      'while',
      'try',
      'except',
      'finally',
      'with',
      'as',
      'import',
      'from',
      'return',
      'yield',
      'break',
      'continue',
      'pass',
      'raise',
      'assert',
      'del',
      'global',
      'nonlocal',
      'lambda',
      'True',
      'False',
      'None',
      'and',
      'or',
      'not',
      'in',
      'is'
    };
    return keywords.contains(text);
  }

  bool _isOperator(String char) {
    const operators = {
      '+',
      '-',
      '*',
      '/',
      '//',
      '%',
      '**',
      '==',
      '!=',
      '<',
      '>',
      '<=',
      '>=',
      '=',
      '+=',
      '-=',
      '*=',
      '/=',
      '//=',
      '%=',
      '**=',
      '&',
      '|',
      '^',
      '~',
      '<<',
      '>>',
      '&=',
      '|=',
      '^=',
      '<<=',
      '>>=',
      'and',
      'or',
      'not',
      'is',
      'in'
    };
    return operators.contains(char);
  }

  bool _isPunctuation(String char) {
    const punctuation = {
      '(',
      ')',
      '[',
      ']',
      '{',
      '}',
      ',',
      ':',
      ';',
      '.',
      '@',
      '->'
    };
    return punctuation.contains(char);
  }

  bool _isWhitespace(String char) {
    return char.isEmpty || char.codeUnits.every((c) => c <= 32);
  }

  bool _isDigit(String char) {
    final code = char.codeUnitAt(0);
    return code >= 48 && code <= 57;
  }

  bool _isIdentifierStart(String char) {
    final code = char.codeUnitAt(0);
    return (code >= 65 && code <= 90) ||
        (code >= 97 && code <= 122) ||
        code == 95;
  }

  bool _isIdentifierPart(String char) {
    return _isIdentifierStart(char) || _isDigit(char);
  }

  /// 清理缓存
  void clearCache() {
    _tokenCache.clear();
    _symbolCache.clear();
    _foldCache.clear();
  }

  /// 清理资源
  void dispose() {
    _queryEngine?.dispose();
    clearCache();
  }
}
