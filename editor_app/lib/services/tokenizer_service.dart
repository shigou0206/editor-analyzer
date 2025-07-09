import '../models/token.dart';

/// Tokenizer 服务 - 文本 → Token 列表转换器
class TokenizerService {
  // Python 关键字集合
  static const Set<String> _keywords = {
    'def',
    'class',
    'if',
    'else',
    'elif',
    'for',
    'while',
    'try',
    'except',
    'finally',
    'with',
    'as',
    'import',
    'from',
    'return',
    'yield',
    'break',
    'continue',
    'pass',
    'raise',
    'assert',
    'del',
    'global',
    'nonlocal',
    'lambda',
    'True',
    'False',
    'None',
    'and',
    'or',
    'not',
    'in',
    'is',
  };

  /// 主要方法：输入字符串，输出 Token 列表
  List<Token> tokenize(String text) {
    final tokens = <Token>[];
    int position = 0;

    while (position < text.length) {
      final char = text[position];

      // 1. 跳过空白字符
      if (_isWhitespace(char)) {
        final start = position;
        while (position < text.length && _isWhitespace(text[position])) {
          position++;
        }
        tokens.add(Token(
          kind: TokenKind.whitespace,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 2. 处理注释 (#)
      if (char == '#') {
        final start = position;
        while (position < text.length && text[position] != '\n') {
          position++;
        }
        tokens.add(Token(
          kind: TokenKind.comment,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 3. 处理字符串 (" 或 ')
      if (char == '"' || char == "'") {
        final start = position;
        final quote = char;
        position++; // 跳过开始引号

        while (position < text.length) {
          if (text[position] == quote) {
            position++; // 包含结束引号
            break;
          }
          if (text[position] == '\\' && position + 1 < text.length) {
            position += 2; // 跳过转义字符
          } else {
            position++;
          }
        }

        tokens.add(Token(
          kind: TokenKind.string,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 4. 处理数字
      if (_isDigit(char)) {
        final start = position;
        while (position < text.length &&
            (_isDigit(text[position]) || text[position] == '.')) {
          position++;
        }
        tokens.add(Token(
          kind: TokenKind.number,
          start: start,
          end: position,
          text: text.substring(start, position),
        ));
        continue;
      }

      // 5. 处理标识符和关键字
      if (_isIdentifierStart(char)) {
        final start = position;
        while (position < text.length && _isIdentifierPart(text[position])) {
          position++;
        }
        final identifier = text.substring(start, position);

        tokens.add(Token(
          kind: _keywords.contains(identifier)
              ? TokenKind.keyword
              : TokenKind.identifier,
          start: start,
          end: position,
          text: identifier,
        ));
        continue;
      }

      // 6. 处理操作符
      if (_isOperator(char)) {
        tokens.add(Token(
          kind: TokenKind.operator,
          start: position,
          end: position + 1,
          text: char,
        ));
        position++;
        continue;
      }

      // 7. 处理标点符号
      if (_isPunctuation(char)) {
        tokens.add(Token(
          kind: TokenKind.punctuation,
          start: position,
          end: position + 1,
          text: char,
        ));
        position++;
        continue;
      }

      // 8. 未知字符
      tokens.add(Token(
        kind: TokenKind.unknown,
        start: position,
        end: position + 1,
        text: char,
      ));
      position++;
    }

    return tokens;
  }

  // 辅助方法
  bool _isWhitespace(String char) {
    return char == ' ' || char == '\t' || char == '\n' || char == '\r';
  }

  bool _isDigit(String char) {
    final code = char.codeUnitAt(0);
    return code >= 48 && code <= 57; // 0-9
  }

  bool _isIdentifierStart(String char) {
    final code = char.codeUnitAt(0);
    return (code >= 65 && code <= 90) || // A-Z
        (code >= 97 && code <= 122) || // a-z
        code == 95; // _
  }

  bool _isIdentifierPart(String char) {
    return _isIdentifierStart(char) || _isDigit(char);
  }

  bool _isOperator(String char) {
    return '+-*/=<>!&|%^~'.contains(char);
  }

  bool _isPunctuation(String char) {
    return '()[]{},.;:'.contains(char);
  }
}
