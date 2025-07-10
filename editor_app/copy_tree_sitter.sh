#!/bin/bash

# 自动复制 Tree-sitter 文件到 Flutter 应用目录
echo "🔄 复制 Tree-sitter 文件到 Flutter 应用目录..."

# 目标目录
TARGET_DIR="build/macos/Build/Products/Debug/editor_app.app/Contents/MacOS"

# 确保目标目录存在
mkdir -p "$TARGET_DIR"

# 复制动态库文件
cp macos/Runner/libtree-sitter.dylib "$TARGET_DIR/"
cp macos/Runner/libtree_sitter_python.dylib "$TARGET_DIR/"

# 复制查询文件
cp macos/Runner/*.scm "$TARGET_DIR/"

echo "✅ Tree-sitter 文件复制完成！"
echo "📁 目标目录: $TARGET_DIR"
ls -la "$TARGET_DIR"/*.dylib "$TARGET_DIR"/*.scm 2>/dev/null || echo "⚠️  部分文件可能不存在" 