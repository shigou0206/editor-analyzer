import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'tree_sitter_bindings.dart';
import 'query_engine.dart';

/// 行 Token 信息
class LineToken {
  final int line;
  final int start;
  final int end;
  final String text;
  final String type;
  final String captureName;

  LineToken({
    required this.line,
    required this.start,
    required this.end,
    required this.text,
    required this.type,
    required this.captureName,
  });

  @override
  String toString() => 'LineToken(line:$line, $type:$text, $start-$end)';
}

/// 行 Token 器
/// 将语法树按行分割，提取每行的 token 信息
class LineTokenizer {
  final TreeSitter bindings;
  final TreeSitterQueryEngine queryEngine;

  LineTokenizer(this.bindings, this.queryEngine);

  /// 按行获取 token
  List<List<LineToken>> getTokensByLine(
    Pointer<TSTree> tree,
    String sourceCode,
    Pointer<TSLanguage> language,
  ) {
    final lines = sourceCode.split('\n');
    final lineTokens = <List<LineToken>>[];

    // 初始化每行的 token 列表
    for (int i = 0; i < lines.length; i++) {
      lineTokens.add([]);
    }

    // 获取所有高亮 token
    try {
      final highlightsQuery = queryEngine.loadQuery('highlights.scm', language);
      final highlights =
          queryEngine.executeHighlights(highlightsQuery, tree, sourceCode);

      // 将 token 分配到对应的行
      for (final highlight in highlights) {
        final line = _getLineNumber(sourceCode, highlight.start);
        if (line >= 0 && line < lines.length) {
          final token = LineToken(
            line: line,
            start: highlight.start,
            end: highlight.end,
            text: sourceCode.substring(highlight.start, highlight.end),
            type: 'highlight',
            captureName: highlight.captureName,
          );
          lineTokens[line].add(token);
        }
      }
    } catch (e) {
      print('⚠️  高亮查询失败: $e');
    }

    // 获取所有标签 token
    try {
      final tagsQuery = queryEngine.loadQuery('tags.scm', language);
      final tags = queryEngine.executeTags(tagsQuery, tree, sourceCode);

      for (final tag in tags) {
        final line = _getLineNumber(sourceCode, tag.start);
        if (line >= 0 && line < lines.length) {
          final token = LineToken(
            line: line,
            start: tag.start,
            end: tag.end,
            text: sourceCode.substring(tag.start, tag.end),
            type: 'tag',
            captureName: tag.type,
          );
          lineTokens[line].add(token);
        }
      }
    } catch (e) {
      print('⚠️  标签查询失败: $e');
    }

    // 按位置排序每行的 token
    for (int i = 0; i < lineTokens.length; i++) {
      lineTokens[i].sort((a, b) => a.start.compareTo(b.start));
    }

    return lineTokens;
  }

  /// 获取指定行的 token
  List<LineToken> getLineTokens(
    Pointer<TSTree> tree,
    String sourceCode,
    Pointer<TSLanguage> language,
    int lineNumber,
  ) {
    final allTokens = getTokensByLine(tree, sourceCode, language);
    if (lineNumber >= 0 && lineNumber < allTokens.length) {
      return allTokens[lineNumber];
    }
    return [];
  }

  /// 获取指定行的语法高亮 token
  List<LineToken> getLineHighlights(
    Pointer<TSTree> tree,
    String sourceCode,
    Pointer<TSLanguage> language,
    int lineNumber,
  ) {
    final lineTokens = getLineTokens(tree, sourceCode, language, lineNumber);
    return lineTokens.where((token) => token.type == 'highlight').toList();
  }

  /// 获取指定行的标签 token
  List<LineToken> getLineTags(
    Pointer<TSTree> tree,
    String sourceCode,
    Pointer<TSLanguage> language,
    int lineNumber,
  ) {
    final lineTokens = getLineTokens(tree, sourceCode, language, lineNumber);
    return lineTokens.where((token) => token.type == 'tag').toList();
  }

  /// 获取指定范围的 token
  List<LineToken> getTokensInRange(
    Pointer<TSTree> tree,
    String sourceCode,
    Pointer<TSLanguage> language,
    int startLine,
    int endLine,
  ) {
    final allTokens = getTokensByLine(tree, sourceCode, language);
    final rangeTokens = <LineToken>[];

    for (int i = startLine; i <= endLine && i < allTokens.length; i++) {
      rangeTokens.addAll(allTokens[i]);
    }

    return rangeTokens;
  }

  /// 获取指定行的基本 token（按空格分割）
  List<LineToken> getBasicTokens(
    String sourceCode,
    int lineNumber,
  ) {
    final lines = sourceCode.split('\n');
    if (lineNumber < 0 || lineNumber >= lines.length) {
      return [];
    }

    final line = lines[lineNumber];
    final tokens = <LineToken>[];
    final words = line.split(RegExp(r'\s+'));

    int currentPos = 0;
    for (final word in words) {
      if (word.isNotEmpty) {
        final startPos = line.indexOf(word, currentPos);
        if (startPos != -1) {
          tokens.add(LineToken(
            line: lineNumber,
            start: startPos,
            end: startPos + word.length,
            text: word,
            type: 'basic',
            captureName: 'word',
          ));
          currentPos = startPos + word.length;
        }
      }
    }

    return tokens;
  }

  /// 获取指定行的所有 token（语法 + 基本）
  List<LineToken> getAllLineTokens(
    Pointer<TSTree> tree,
    String sourceCode,
    Pointer<TSLanguage> language,
    int lineNumber,
  ) {
    final syntaxTokens = getLineTokens(tree, sourceCode, language, lineNumber);
    final basicTokens = getBasicTokens(sourceCode, lineNumber);

    // 合并并去重
    final allTokens = <LineToken>[];
    allTokens.addAll(syntaxTokens);

    // 添加基本 token（如果不在语法 token 中）
    for (final basicToken in basicTokens) {
      bool exists = false;
      for (final syntaxToken in syntaxTokens) {
        if (syntaxToken.start <= basicToken.start &&
            syntaxToken.end >= basicToken.end) {
          exists = true;
          break;
        }
      }
      if (!exists) {
        allTokens.add(basicToken);
      }
    }

    // 按位置排序
    allTokens.sort((a, b) => a.start.compareTo(b.start));
    return allTokens;
  }

  /// 计算字符位置对应的行号
  int _getLineNumber(String sourceCode, int charPosition) {
    if (charPosition < 0 || charPosition >= sourceCode.length) {
      return -1;
    }

    int line = 0;
    for (int i = 0; i < charPosition; i++) {
      if (sourceCode[i] == '\n') {
        line++;
      }
    }
    return line;
  }

  /// 获取 token 的详细信息
  Map<String, dynamic> getTokenInfo(LineToken token) {
    return {
      'line': token.line,
      'start': token.start,
      'end': token.end,
      'text': token.text,
      'type': token.type,
      'captureName': token.captureName,
      'length': token.text.length,
    };
  }

  /// 格式化显示 token
  String formatToken(LineToken token) {
    return '${token.type}:${token.captureName} "${token.text}" (${token.start}-${token.end})';
  }
}
