import 'package:flutter/material.dart';
import '../models/token.dart';

/// 语法高亮器 - Token → TextSpan 转换器
class SyntaxHighlighter {
  // VS Code 风格的颜色方案
  static const Map<TokenKind, TextStyle> _tokenStyles = {
    TokenKind.keyword: TextStyle(
      color: Color(0xFF569CD6), // 蓝色 - 关键字
      fontWeight: FontWeight.w500,
    ),
    TokenKind.identifier: TextStyle(
      color: Color(0xFF9CDCFE), // 浅蓝色 - 标识符
    ),
    TokenKind.string: TextStyle(
      color: Color(0xFFCE9178), // 橙色 - 字符串
    ),
    TokenKind.number: TextStyle(
      color: Color(0xFFB5CEA8), // 绿色 - 数字
    ),
    TokenKind.comment: TextStyle(
      color: Color(0xFF6A9955), // 绿色 - 注释
      fontStyle: FontStyle.italic,
    ),
    TokenKind.punctuation: TextStyle(
      color: Color(0xFFD4D4D4), // 白色 - 标点符号
    ),
    TokenKind.operator: TextStyle(
      color: Color(0xFFD4D4D4), // 白色 - 操作符
    ),
    TokenKind.whitespace: TextStyle(
      color: Color(0xFFD4D4D4), // 默认颜色 - 空白字符
    ),
    TokenKind.unknown: TextStyle(
      color: Color(0xFFD4D4D4), // 默认颜色 - 未知字符
    ),
  };

  // 默认文本样式
  static const TextStyle _defaultStyle = TextStyle(
    color: Color(0xFFD4D4D4), // VS Code 默认前景色
    fontSize: 14,
    fontFamily: 'Courier', // 使用等宽字体
    height: 1.4,
  );

  /// 主要方法：将文本和 Token 列表转换为 TextSpan 列表
  List<TextSpan> buildTokenSpans(String text, List<Token> tokens) {
    final spans = <TextSpan>[];
    int currentPosition = 0;

    for (final token in tokens) {
      // 处理 token 之前的未匹配文本
      if (token.start > currentPosition) {
        final unmatchedText = text.substring(currentPosition, token.start);
        spans.add(TextSpan(
          text: unmatchedText,
          style: _defaultStyle,
        ));
      }

      // 对于空格token，强制使用等宽字体
      if (token.kind == TokenKind.whitespace) {
        spans.add(TextSpan(
          text: token.text,
          style: _defaultStyle.copyWith(
            fontFamily: 'Courier',
            letterSpacing: 0.0, // 确保字符间距正常
          ),
        ));
      } else {
        // 添加当前 token 的样式化文本
        final tokenStyle = _tokenStyles[token.kind] ?? _defaultStyle;
        spans.add(TextSpan(
          text: token.text,
          style: _defaultStyle.merge(tokenStyle),
        ));
      }

      currentPosition = token.end;
    }

    // 处理剩余的未匹配文本
    if (currentPosition < text.length) {
      final remainingText = text.substring(currentPosition);
      spans.add(TextSpan(
        text: remainingText,
        style: _defaultStyle,
      ));
    }

    return spans;
  }

  /// 获取指定 Token 类型的样式
  TextStyle getStyleForToken(TokenKind kind) {
    return _defaultStyle.merge(_tokenStyles[kind] ?? const TextStyle());
  }

  /// 获取默认样式
  TextStyle get defaultStyle => _defaultStyle;
}
