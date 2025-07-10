import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'src/tree_sitter_bindings.dart';

/// Tree-sitter 查询引擎
/// 支持语法高亮、符号导航、代码折叠、标签跳转
class TreeSitterQueryEngine {
  final TreeSitter bindings;
  final DynamicLibrary treeSitterLib;

  // 查询缓存
  Map<String, Pointer<TSQuery>> _queryCache = {};

  TreeSitterQueryEngine(this.bindings, this.treeSitterLib);

  /// 加载查询文件
  Pointer<TSQuery> loadQuery(String queryFile, Pointer<TSLanguage> language) {
    if (_queryCache.containsKey(queryFile)) {
      return _queryCache[queryFile]!;
    }

    final file = File(queryFile);
    if (!file.existsSync()) {
      throw Exception('查询文件不存在: $queryFile');
    }

    final querySource = file.readAsStringSync();
    final sourcePtr = querySource.toNativeUtf8();

    final errorOffset = calloc<Uint32>();
    final errorType = calloc<UnsignedInt>();

    final query = bindings.ts_query_new(
      language,
      sourcePtr.cast(),
      querySource.length,
      errorOffset,
      errorType,
    );

    calloc.free(sourcePtr);

    if (query.address == 0) {
      final error =
          _getQueryErrorString(TSQueryError.fromValue(errorType.value));
      calloc.free(errorOffset);
      calloc.free(errorType);
      throw Exception('查询编译失败: $error');
    }

    calloc.free(errorOffset);
    calloc.free(errorType);

    _queryCache[queryFile] = query;
    return query;
  }

  /// 执行语法高亮查询
  List<Highlight> executeHighlights(
    Pointer<TSQuery> query,
    Pointer<TSTree> tree,
    String sourceCode,
  ) {
    final highlights = <Highlight>[];

    if (query.address == 0 || tree.address == 0) {
      return highlights;
    }

    final queryCursor = bindings.ts_query_cursor_new();
    if (queryCursor.address == 0) {
      return highlights;
    }

    try {
      bindings.ts_query_cursor_exec(
          queryCursor, query, bindings.ts_tree_root_node(tree));

      final match = calloc<TSQueryMatch>();
      final captureIndex = calloc<Uint32>();

      try {
        while (bindings.ts_query_cursor_next_match(queryCursor, match)) {
          for (int i = 0; i < match.ref.capture_count; i++) {
            final capture = match.ref.captures[i];
            final node = capture.node;

            if (node.id.address == 0) continue;

            final startByte = bindings.ts_node_start_byte(node);
            final endByte = bindings.ts_node_end_byte(node);

            if (startByte < 0 ||
                endByte < startByte ||
                endByte > sourceCode.length) {
              continue;
            }

            // 计算字符位置
            final startChar = _byteToChar(sourceCode, startByte);
            final endChar = _byteToChar(sourceCode, endByte);

            highlights.add(Highlight(
              start: startChar,
              end: endChar,
              captureName: _getCaptureName(query, capture.index),
            ));
          }
        }
      } finally {
        calloc.free(match);
        calloc.free(captureIndex);
      }
    } finally {
      bindings.ts_query_cursor_delete(queryCursor);
    }

    return highlights;
  }

  /// 执行符号查询（locals.scm）
  List<Symbol> executeLocals(
    Pointer<TSQuery> query,
    Pointer<TSTree> tree,
    String sourceCode,
  ) {
    final symbols = <Symbol>[];
    final queryCursor = bindings.ts_query_cursor_new();

    bindings.ts_query_cursor_exec(
        queryCursor, query, bindings.ts_tree_root_node(tree));

    final match = calloc<TSQueryMatch>();

    while (bindings.ts_query_cursor_next_match(queryCursor, match)) {
      for (int i = 0; i < match.ref.capture_count; i++) {
        final capture = match.ref.captures[i];
        final node = capture.node;

        final startByte = bindings.ts_node_start_byte(node);
        final endByte = bindings.ts_node_end_byte(node);
        final startChar = _byteToChar(sourceCode, startByte);
        final endChar = _byteToChar(sourceCode, endByte);

        final name = sourceCode.substring(startChar, endChar);
        final captureName = _getCaptureName(query, capture.index);

        symbols.add(Symbol(
          name: name,
          type: captureName,
          start: startChar,
          end: endChar,
          node: node.id,
        ));
      }
    }

    calloc.free(match);
    bindings.ts_query_cursor_delete(queryCursor);

    return symbols;
  }

  /// 执行折叠查询（folds.scm）
  List<Fold> executeFolds(
    Pointer<TSQuery> query,
    Pointer<TSTree> tree,
    String sourceCode,
  ) {
    final folds = <Fold>[];
    final queryCursor = bindings.ts_query_cursor_new();

    bindings.ts_query_cursor_exec(
        queryCursor, query, bindings.ts_tree_root_node(tree));

    final match = calloc<TSQueryMatch>();

    while (bindings.ts_query_cursor_next_match(queryCursor, match)) {
      for (int i = 0; i < match.ref.capture_count; i++) {
        final capture = match.ref.captures[i];
        final node = capture.node;

        final startByte = bindings.ts_node_start_byte(node);
        final endByte = bindings.ts_node_end_byte(node);
        final startChar = _byteToChar(sourceCode, startByte);
        final endChar = _byteToChar(sourceCode, endByte);

        folds.add(Fold(
          start: startChar,
          end: endChar,
          type: _getCaptureName(query, capture.index),
        ));
      }
    }

    calloc.free(match);
    bindings.ts_query_cursor_delete(queryCursor);

    return folds;
  }

  /// 执行标签查询（tags.scm）
  List<Tag> executeTags(
    Pointer<TSQuery> query,
    Pointer<TSTree> tree,
    String sourceCode,
  ) {
    final tags = <Tag>[];
    final queryCursor = bindings.ts_query_cursor_new();

    bindings.ts_query_cursor_exec(
        queryCursor, query, bindings.ts_tree_root_node(tree));

    final match = calloc<TSQueryMatch>();

    while (bindings.ts_query_cursor_next_match(queryCursor, match)) {
      for (int i = 0; i < match.ref.capture_count; i++) {
        final capture = match.ref.captures[i];
        final node = capture.node;

        final startByte = bindings.ts_node_start_byte(node);
        final endByte = bindings.ts_node_end_byte(node);
        final startChar = _byteToChar(sourceCode, startByte);
        final endChar = _byteToChar(sourceCode, endByte);

        final name = sourceCode.substring(startChar, endChar);
        final captureName = _getCaptureName(query, capture.index);

        tags.add(Tag(
          name: name,
          type: captureName,
          start: startChar,
          end: endChar,
        ));
      }
    }

    calloc.free(match);
    bindings.ts_query_cursor_delete(queryCursor);

    return tags;
  }

  /// 获取捕获名称
  String _getCaptureName(Pointer<TSQuery> query, int index) {
    if (query.address == 0) return 'unknown';

    // 检查索引是否有效
    final captureCount = bindings.ts_query_capture_count(query);
    if (index < 0 || index >= captureCount) {
      return 'unknown_$index';
    }

    // 暂时跳过 ts_query_capture_name_for_id 调用，直接使用索引
    // 因为该函数在某些情况下会导致崩溃
    return 'capture_$index';

    /*
    final namePtr = bindings.ts_query_capture_name_for_id(query, index, nullptr);
    if (namePtr.address == 0) return 'unknown';
    
    try {
      return namePtr.cast<Utf8>().toDartString();
    } catch (e) {
      return 'unknown';
    }
    */
  }

  /// 获取查询错误信息
  String _getQueryErrorString(TSQueryError error) {
    switch (error) {
      case TSQueryError.TSQueryErrorNone:
        return '无错误';
      case TSQueryError.TSQueryErrorSyntax:
        return '语法错误';
      case TSQueryError.TSQueryErrorNodeType:
        return '节点类型错误';
      case TSQueryError.TSQueryErrorField:
        return '字段错误';
      case TSQueryError.TSQueryErrorCapture:
        return '捕获错误';
      case TSQueryError.TSQueryErrorStructure:
        return '结构错误';
      case TSQueryError.TSQueryErrorLanguage:
        return '语言错误';
      default:
        return '未知错误';
    }
  }

  /// 字节位置转换为字符位置
  int _byteToChar(String text, int byteOffset) {
    return text.substring(0, byteOffset).length;
  }

  /// 清理资源
  void dispose() {
    for (final query in _queryCache.values) {
      bindings.ts_query_delete(query);
    }
    _queryCache.clear();
  }
}

/// 语法高亮信息
class Highlight {
  final int start;
  final int end;
  final String captureName;

  Highlight({
    required this.start,
    required this.end,
    required this.captureName,
  });

  @override
  String toString() => 'Highlight($start-$end: $captureName)';
}

/// 符号信息
class Symbol {
  final String name;
  final String type;
  final int start;
  final int end;
  final Pointer<Void> node;

  Symbol({
    required this.name,
    required this.type,
    required this.start,
    required this.end,
    required this.node,
  });

  @override
  String toString() => 'Symbol($name: $type at $start-$end)';
}

/// 折叠信息
class Fold {
  final int start;
  final int end;
  final String type;

  Fold({
    required this.start,
    required this.end,
    required this.type,
  });

  @override
  String toString() => 'Fold($start-$end: $type)';
}

/// 标签信息
class Tag {
  final String name;
  final String type;
  final int start;
  final int end;

  Tag({
    required this.name,
    required this.type,
    required this.start,
    required this.end,
  });

  @override
  String toString() => 'Tag($name: $type at $start-$end)';
}
