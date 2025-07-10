import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'src/tree_sitter_bindings.dart';
import 'query_engine.dart';

void main() {
  print('🚀 简单查询测试...');

  try {
    // 加载库
    final treeSitterLib = DynamicLibrary.open('libtree-sitter.dylib');
    final bindings = TreeSitter(treeSitterLib);

    final pythonLib = DynamicLibrary.open('libtree_sitter_python.dylib');
    final treeSitterPython = pythonLib
        .lookup<NativeFunction<Pointer<Void> Function()>>('tree_sitter_python')
        .asFunction<Pointer<Void> Function()>()();

    // 创建解析器
    final parser = bindings.ts_parser_new();
    final success =
        bindings.ts_parser_set_language(parser, treeSitterPython.cast());
    if (!success) throw Exception('❌ 设置语言失败');

    // 简单的测试代码
    final sourceCode = 'def hello(): return "world"';

    // 解析代码
    final codePtr = sourceCode.toNativeUtf8();
    final tree = bindings.ts_parser_parse_string(
      parser,
      nullptr,
      codePtr.cast(),
      sourceCode.length,
    );
    calloc.free(codePtr);

    if (tree.address == 0) throw Exception('❌ 解析失败');
    print('✅ 代码解析成功');

    // 测试查询引擎创建
    print('\n🔧 测试查询引擎创建...');
    final queryEngine = TreeSitterQueryEngine(bindings, treeSitterLib);
    print('✅ 查询引擎创建成功');

    // 测试查询文件加载
    print('\n📁 测试查询文件加载...');
    try {
      final highlightsQuery =
          queryEngine.loadQuery('highlights.scm', treeSitterPython.cast());
      print('✅ highlights.scm 加载成功');

      // 测试基本查询执行
      print('\n🔍 测试基本查询执行...');
      final highlights =
          queryEngine.executeHighlights(highlightsQuery, tree, sourceCode);
      print('✅ 查询执行成功，找到 ${highlights.length} 个高亮');

      for (final highlight in highlights.take(5)) {
        final text = sourceCode.substring(highlight.start, highlight.end);
        print(
            '  ${highlight.captureName}: "${text}" (${highlight.start}-${highlight.end})');
      }
    } catch (e) {
      print('❌ 查询测试失败: $e');
    }

    // 清理资源
    queryEngine.dispose();
    bindings.ts_tree_delete(tree);
    bindings.ts_parser_delete(parser);

    print('\n🎉 简单查询测试完成！');
  } catch (e) {
    print('❌ 错误: $e');
    exit(1);
  }
}
