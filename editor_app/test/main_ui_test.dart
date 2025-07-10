import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:editor_app/main.dart';
import 'package:editor_app/services/tree_sitter_enhanced.dart';
import 'package:editor_app/widgets/vs_code_editor.dart';
import 'package:editor_app/models/token.dart';

void main() {
  group('Main UI Tree-sitter Integration Tests', () {
    testWidgets('Tree-sitter initialization and syntax highlighting',
        (WidgetTester tester) async {
      // 构建应用
      await tester.pumpWidget(const MyApp());

      // 等待应用初始化
      await tester.pumpAndSettle();

      // 验证 Tree-sitter 是否初始化成功
      final treeSitter = TreeSitterEnhanced.instance;
      expect(treeSitter, isNotNull);

      // 测试语法高亮
      final testCode = '''
def test_function():
    name = "World"
    print(f"Hello {name}!")
    return f"Count: {42}"
''';

      final tokens = treeSitter.highlight(testCode);
      expect(tokens, isNotEmpty);

      // 验证关键 token 类型
      final keywordTokens =
          tokens.where((t) => t.kind == TokenKind.keyword).toList();
      final stringTokens =
          tokens.where((t) => t.kind == TokenKind.string).toList();
      final identifierTokens =
          tokens.where((t) => t.kind == TokenKind.identifier).toList();

      expect(keywordTokens, isNotEmpty);
      expect(stringTokens, isNotEmpty);
      expect(identifierTokens, isNotEmpty);

      print('✅ Tree-sitter 语法高亮测试通过');
      print('  - 关键字数量: ${keywordTokens.length}');
      print('  - 字符串数量: ${stringTokens.length}');
      print('  - 标识符数量: ${identifierTokens.length}');

      // 测试符号导航
      final symbols = treeSitter.getSymbols(testCode);
      expect(symbols, isNotEmpty);

      print('✅ 符号导航测试通过');
      print('  - 符号数量: ${symbols.length}');

      // 测试代码折叠
      final folds = treeSitter.getFolds(testCode);
      expect(folds, isNotEmpty);

      print('✅ 代码折叠测试通过');
      print('  - 折叠区域数量: ${folds.length}');
    });

    testWidgets('Editor widget with Tree-sitter highlighting',
        (WidgetTester tester) async {
      // 构建编辑器
      await tester.pumpWidget(MaterialApp(
        home: Scaffold(
          body: VSCodeEditor(
            initialCode: '''
def hello_world():
    name = "Flutter"
    print(f"Hello {name}!")
    return "Success"
''',
            onChanged: (code) {
              print('代码变更: $code');
            },
          ),
        ),
      ));

      // 等待编辑器渲染
      await tester.pumpAndSettle();

      // 验证编辑器是否正常显示
      expect(find.byType(VSCodeEditor), findsOneWidget);

      // 验证语法高亮是否应用
      final textWidgets = find.byType(Text);
      expect(textWidgets, findsWidgets);

      print('✅ 编辑器 UI 测试通过');
    });

    test('Tree-sitter f-string highlighting', () {
      final treeSitter = TreeSitterEnhanced.instance;

      final fStringCode = '''
def test_f_strings():
    name = "Alice"
    age = 25
    print(f"Name: {name}, Age: {age}")
    return f"Result: {name} is {age} years old"
''';

      final tokens = treeSitter.highlight(fStringCode);

      // 验证 f-string 被正确识别
      final stringTokens =
          tokens.where((t) => t.kind == TokenKind.string).toList();
      expect(stringTokens, isNotEmpty);

      // 检查是否包含 f-string
      final fStringTexts = stringTokens.map((t) => t.text).toList();
      final hasFString = fStringTexts.any((text) => text.startsWith('f"'));

      expect(hasFString, isTrue);

      print('✅ F-string 高亮测试通过');
      print('  - 字符串 token: ${fStringTexts.join(', ')}');
    });

    test('Tree-sitter error handling', () {
      final treeSitter = TreeSitterEnhanced.instance;

      // 测试无效代码
      final invalidCode = '''
def invalid_syntax:
    print("Missing colon"
    return "Unclosed string
''';

      // 应该能处理语法错误而不崩溃
      final tokens = treeSitter.highlight(invalidCode);
      expect(tokens, isNotEmpty);

      print('✅ 错误处理测试通过');
    });
  });
}
