import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'src/tree_sitter_bindings.dart';
import 'query_engine.dart';

void main() {
  print('🚀 Tree-sitter 查询引擎示例...');

  try {
    // 加载库
    final treeSitterLib = DynamicLibrary.open('libtree-sitter.dylib');
    final bindings = TreeSitter(treeSitterLib);

    final pythonLib = DynamicLibrary.open('libtree_sitter_python.dylib');
    final treeSitterPython = pythonLib
        .lookup<NativeFunction<Pointer<Void> Function()>>('tree_sitter_python')
        .asFunction<Pointer<Void> Function()>()();

    // 创建查询引擎
    final queryEngine = TreeSitterQueryEngine(bindings, treeSitterLib);

    // 创建解析器
    final parser = bindings.ts_parser_new();
    final success =
        bindings.ts_parser_set_language(parser, treeSitterPython.cast());
    if (!success) throw Exception('❌ 设置语言失败');

    // 测试代码
    final sourceCode = '''
class Calculator:
    """简单的计算器类"""
    
    def __init__(self):
        self.result = 0
    
    def add(self, a, b):
        """加法运算"""
        return a + b
    
    def multiply(self, x, y):
        """乘法运算"""
        return x * y

def fibonacci(n):
    """计算斐波那契数列"""
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

# 主函数
def main():
    calc = Calculator()
    result = calc.add(10, 20)
    print(f"结果: {result}")
    
    fib = fibonacci(10)
    print(f"斐波那契数: {fib}")

if __name__ == "__main__":
    main()
''';

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

    // 1. 语法高亮查询
    print('\n🎨 语法高亮查询:');
    try {
      final highlightsQuery =
          queryEngine.loadQuery('highlights.scm', treeSitterPython.cast());
      final highlights =
          queryEngine.executeHighlights(highlightsQuery, tree, sourceCode);

      print('找到 ${highlights.length} 个高亮区域:');
      for (final highlight in highlights.take(10)) {
        // 只显示前10个
        final text = sourceCode.substring(highlight.start, highlight.end);
        print(
            '  ${highlight.captureName}: "${text}" (${highlight.start}-${highlight.end})');
      }
    } catch (e) {
      print('⚠️  语法高亮查询失败: $e');
    }

    // 2. 符号查询（locals.scm）
    print('\n🔍 符号查询:');
    try {
      final localsQuery =
          queryEngine.loadQuery('locals.scm', treeSitterPython.cast());
      final symbols = queryEngine.executeLocals(localsQuery, tree, sourceCode);

      print('找到 ${symbols.length} 个符号:');
      for (final symbol in symbols) {
        print(
            '  ${symbol.type}: ${symbol.name} (${symbol.start}-${symbol.end})');
      }
    } catch (e) {
      print('⚠️  符号查询失败: $e');
    }

    // 3. 折叠查询（folds.scm）
    print('\n📁 代码折叠查询:');
    try {
      final foldsQuery =
          queryEngine.loadQuery('folds.scm', treeSitterPython.cast());
      final folds = queryEngine.executeFolds(foldsQuery, tree, sourceCode);

      print('找到 ${folds.length} 个可折叠区域:');
      for (final fold in folds) {
        final text = sourceCode.substring(fold.start, fold.end);
        print(
            '  ${fold.type}: "${text.substring(0, text.length > 50 ? 50 : text.length)}..." (${fold.start}-${fold.end})');
      }
    } catch (e) {
      print('⚠️  折叠查询失败: $e');
    }

    // 4. 标签查询（tags.scm）
    print('\n🏷️  标签查询:');
    try {
      final tagsQuery =
          queryEngine.loadQuery('tags.scm', treeSitterPython.cast());
      final tags = queryEngine.executeTags(tagsQuery, tree, sourceCode);

      print('找到 ${tags.length} 个标签:');
      for (final tag in tags) {
        print('  ${tag.type}: ${tag.name} (${tag.start}-${tag.end})');
      }
    } catch (e) {
      print('⚠️  标签查询失败: $e');
    }

    // 清理资源
    queryEngine.dispose();
    bindings.ts_tree_delete(tree);
    bindings.ts_parser_delete(parser);

    print('\n🎉 查询引擎示例完成！');
  } catch (e) {
    print('❌ 错误: $e');
    exit(1);
  }
}
