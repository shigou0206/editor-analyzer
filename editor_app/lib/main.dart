import 'package:flutter/material.dart';
import 'widgets/vs_code_editor.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'VS Code Editor Demo',
      theme: ThemeData.dark(),
      home: const EditorPage(),
    );
  }
}

class EditorPage extends StatefulWidget {
  const EditorPage({super.key});

  @override
  State<EditorPage> createState() => _EditorPageState();
}

class _EditorPageState extends State<EditorPage> {
  String _code = '''def hello_world():
    """A simple greeting function"""
    name = "World"
    print(f"Hello, {name}!")
    
    # Calculate something
    for i in range(3):
        print(f"Count: {i}")
    
    return "Done"

# Call the function
result = hello_world()
''';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF1E1E1E),
      body: VSCodeEditor(
        initialCode: _code,
        onChanged: (newCode) => setState(() => _code = newCode),
      ),
    );
  }
}
