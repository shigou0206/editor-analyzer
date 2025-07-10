import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'src/tree_sitter_bindings.dart';

void main() {
  print('🚀 测试 Tree-sitter Python 绑定...');
  try {
    final treeSitterLib = DynamicLibrary.open('libtree-sitter.dylib');
    final bindings = TreeSitter(treeSitterLib);
    final pythonLib = DynamicLibrary.open('libtree_sitter_python.dylib');
    final treeSitterPython = pythonLib
        .lookup<NativeFunction<Pointer<Void> Function()>>('tree_sitter_python')
        .asFunction<Pointer<Void> Function()>()();
    final parser = bindings.ts_parser_new();
    print('✅ 解析器创建成功');
    final success = bindings.ts_parser_set_language(parser, treeSitterPython.cast());
    if (!success) throw Exception('❌ 设置语言失败');
    final sourceCode = '''def foo():\n  return 42''';
    final codePtr = sourceCode.toNativeUtf8();
    final tree = bindings.ts_parser_parse_string(
      parser,
      nullptr,
      codePtr.cast(),
      sourceCode.length,
    );
    calloc.free(codePtr);
    if (tree.address == 0) throw Exception('❌ 解析失败');
    print('✅ 解析成功');
    final rootNode = bindings.ts_tree_root_node(tree);
    final typePtr = bindings.ts_node_type(rootNode);
    final typeStr = typePtr.cast<Utf8>().toDartString();
    print('Root node type: ' + typeStr);
    bindings.ts_tree_delete(tree);
    bindings.ts_parser_delete(parser);
    print('🎉 所有测试通过！Tree-sitter Python 绑定工作正常。');
  } catch (e) {
    print('❌ 错误: ' + e.toString());
    exit(1);
  }
}
