#!/bin/bash
set -e

PROJECT_NAME="tree_sitter_binding"
LANG_NAME="python"
LIB_NAME="tree_sitter_${LANG_NAME}"
DYLIB_NAME="lib${LIB_NAME}.dylib"

# 1️⃣ 克隆 tree-sitter（主项目）
if [ ! -d "tree-sitter" ]; then
  echo "📥 克隆 tree-sitter..."
  git clone https://github.com/tree-sitter/tree-sitter.git
fi

# 2️⃣ 克隆 tree-sitter-python（语言 grammar）
if [ ! -d "tree-sitter-$LANG_NAME" ]; then
  echo "📥 克隆 tree-sitter-$LANG_NAME..."
  git clone https://github.com/tree-sitter/tree-sitter-$LANG_NAME.git
fi

# 3️⃣ 构建 tree-sitter 主库动态库
cd tree-sitter/lib
mkdir -p ../../$PROJECT_NAME
if [ ! -f "libtree-sitter.dylib" ]; then
  echo "🔧 编译 tree-sitter 主库为动态库..."
  gcc -fPIC -shared -I./include -I./src src/lib.c -o libtree-sitter.dylib
fi
cp libtree-sitter.dylib ../../$PROJECT_NAME/
cd ../../

# 4️⃣ 构建 grammar 动态库
cd tree-sitter-$LANG_NAME
if [ ! -f "$DYLIB_NAME" ]; then
  echo "🔧 生成 parser.c ..."
  npx tree-sitter generate
  echo "🔧 编译 $DYLIB_NAME ..."
  if [ -f src/scanner.c ]; then
    gcc -fPIC -shared -I./src src/parser.c src/scanner.c -o $DYLIB_NAME
  else
    gcc -fPIC -shared -I./src src/parser.c -o $DYLIB_NAME
  fi
fi
cp $DYLIB_NAME ../$PROJECT_NAME/
cd ..

# 5️⃣ 拷贝 queries 文件
QUERIES=(highlights.scm locals.scm folds.scm tags.scm)
for q in "${QUERIES[@]}"; do
  if [ -f "tree-sitter-$LANG_NAME/queries/$q" ]; then
    cp "tree-sitter-$LANG_NAME/queries/$q" $PROJECT_NAME/
    echo "✅ 拷贝 $q"
  else
    echo "⚠️  $q 不存在，跳过"
  fi
done

# 6️⃣ 拷贝 api.h
cp tree-sitter/lib/include/tree_sitter/api.h $PROJECT_NAME/api.h

# 7️⃣ Dart 工程 scaffold
mkdir -p $PROJECT_NAME/lib/src

cat > $PROJECT_NAME/pubspec.yaml <<EOF
name: $PROJECT_NAME
description: Dart FFI binding for Tree-sitter $LANG_NAME
version: 0.1.0

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  ffi: ^2.0.2

dev_dependencies:
  ffigen: ^9.0.0
EOF

cat > $PROJECT_NAME/ffigen.yaml <<EOF
output: lib/src/tree_sitter_bindings.dart
name: TreeSitter
headers:
  entry-points:
    - api.h
  include-directives:
    - '**/*.h'
compiler-opts:
  - '-I.'
functions:
  include:
    - 'ts_.*'
  exclude:
    - 'ts_language_.*'
structs:
  include:
    - 'TS.*'
typedefs:
  include:
    - 'TS.*'
enums:
  include:
    - 'TS.*'
macros:
  include:
    - 'TREE_SITTER_.*'
EOF

cat > $PROJECT_NAME/lib/example.dart <<EOF
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
EOF

cd $PROJECT_NAME

echo "📦 安装 Dart 依赖..."
dart pub get

echo "⚡️ 全局激活 ffigen..."
dart pub global activate ffigen

echo "⚡️ 生成 Dart FFI 绑定..."
dart pub global run ffigen --config ffigen.yaml

echo "🚀 运行示例..."
dart run lib/example.dart

# 清理临时构建目录
echo "🧹 清理临时构建目录..."
cd ..
rm -rf tree-sitter tree-sitter-python
echo "✅ 构建完成！项目目录: tree_sitter_binding/" 