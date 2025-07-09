/// Token 类型枚举
enum TokenKind {
  keyword, // 关键字：def, class, if, for...
  identifier, // 标识符：变量名、函数名
  string, // 字符串字面量
  number, // 数字字面量
  comment, // 注释
  punctuation, // 标点符号：( ) [ ] { }
  operator, // 操作符：+ - * / =
  whitespace, // 空白字符
  unknown, // 未知字符
}

/// Token 结构表达单位
class Token {
  final TokenKind kind;
  final int start;
  final int end;
  final String text;

  const Token({
    required this.kind,
    required this.start,
    required this.end,
    required this.text,
  });

  @override
  String toString() {
    return 'Token(kind: $kind, start: $start, end: $end, text: "$text")';
  }
}
