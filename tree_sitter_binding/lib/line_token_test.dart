import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'src/tree_sitter_bindings.dart';
import 'query_engine.dart';
import 'line_tokenizer.dart';

void main() {
  print('🚀 按行 Token 测试...');

  try {
    // 加载库
    final treeSitterLib = DynamicLibrary.open('libtree-sitter.dylib');
    final bindings = TreeSitter(treeSitterLib);

    final pythonLib = DynamicLibrary.open('libtree_sitter_python.dylib');
    final treeSitterPython = pythonLib
        .lookup<NativeFunction<Pointer<Void> Function()>>('tree_sitter_python')
        .asFunction<Pointer<Void> Function()>()();

    // 创建查询引擎和行 token 器
    final queryEngine = TreeSitterQueryEngine(bindings, treeSitterLib);
    final lineTokenizer = LineTokenizer(bindings, queryEngine);

    // 创建解析器
    final parser = bindings.ts_parser_new();
    final success =
        bindings.ts_parser_set_language(parser, treeSitterPython.cast());
    if (!success) throw Exception('❌ 设置语言失败');

    // 测试代码
    final sourceCode = '''class Calculator:
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

    // 1. 测试按行获取所有 token
    print('\n📋 按行获取所有 token:');
    final allLineTokens = lineTokenizer.getTokensByLine(
        tree, sourceCode, treeSitterPython.cast());
    print('总行数: ${allLineTokens.length}');

    // 显示前5行的 token
    for (int i = 0; i < allLineTokens.length && i < 5; i++) {
      final tokens = allLineTokens[i];
      print('第 ${i + 1} 行 (${tokens.length} 个 token):');
      for (final token in tokens.take(5)) {
        // 只显示前5个
        print('  ${lineTokenizer.formatToken(token)}');
      }
      if (tokens.length > 5) {
        print('  ... 还有 ${tokens.length - 5} 个 token');
      }
    }

    // 2. 测试获取指定行的 token
    print('\n🎯 获取指定行的 token:');
    final line3Tokens = lineTokenizer.getLineTokens(
        tree, sourceCode, treeSitterPython.cast(), 2); // 第3行
    print('第 3 行的 token (${line3Tokens.length} 个):');
    for (final token in line3Tokens) {
      print('  ${lineTokenizer.formatToken(token)}');
    }

    // 3. 测试获取指定行的语法高亮 token
    print('\n🎨 获取指定行的语法高亮 token:');
    final line1Highlights = lineTokenizer.getLineHighlights(
        tree, sourceCode, treeSitterPython.cast(), 0); // 第1行
    print('第 1 行的高亮 token (${line1Highlights.length} 个):');
    for (final token in line1Highlights) {
      print('  ${lineTokenizer.formatToken(token)}');
    }

    // 4. 测试获取指定行的标签 token
    print('\n🏷️  获取指定行的标签 token:');
    final line1Tags = lineTokenizer.getLineTags(
        tree, sourceCode, treeSitterPython.cast(), 0); // 第1行
    print('第 1 行的标签 token (${line1Tags.length} 个):');
    for (final token in line1Tags) {
      print('  ${lineTokenizer.formatToken(token)}');
    }

    // 5. 测试获取指定范围的 token
    print('\n📊 获取指定范围的 token (第1-3行):');
    final rangeTokens = lineTokenizer.getTokensInRange(
        tree, sourceCode, treeSitterPython.cast(), 0, 2);
    print('第 1-3 行的 token (${rangeTokens.length} 个):');
    for (final token in rangeTokens.take(10)) {
      // 只显示前10个
      print('  ${lineTokenizer.formatToken(token)}');
    }
    if (rangeTokens.length > 10) {
      print('  ... 还有 ${rangeTokens.length - 10} 个 token');
    }

    // 6. 测试获取基本 token（按空格分割）
    print('\n🔤 获取基本 token (按空格分割):');
    final basicTokens = lineTokenizer.getBasicTokens(sourceCode, 0); // 第1行
    print('第 1 行的基本 token (${basicTokens.length} 个):');
    for (final token in basicTokens) {
      print('  ${lineTokenizer.formatToken(token)}');
    }

    // 7. 测试获取所有 token（语法 + 基本）
    print('\n🔄 获取所有 token (语法 + 基本):');
    final allTokens = lineTokenizer.getAllLineTokens(
        tree, sourceCode, treeSitterPython.cast(), 0); // 第1行
    print('第 1 行的所有 token (${allTokens.length} 个):');
    for (final token in allTokens) {
      print('  ${lineTokenizer.formatToken(token)}');
    }

    // 8. 测试 token 详细信息
    print('\n📝 Token 详细信息:');
    if (allTokens.isNotEmpty) {
      final firstToken = allTokens.first;
      final tokenInfo = lineTokenizer.getTokenInfo(firstToken);
      print('第一个 token 的详细信息:');
      tokenInfo.forEach((key, value) {
        print('  $key: $value');
      });
    }

    // 清理资源
    queryEngine.dispose();
    bindings.ts_tree_delete(tree);
    bindings.ts_parser_delete(parser);

    print('\n🎉 按行 Token 测试完成！');
  } catch (e) {
    print('❌ 错误: $e');
    exit(1);
  }
}
