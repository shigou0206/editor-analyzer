import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter/rendering.dart';
import '../services/tree_sitter_enhanced.dart';

class VSCodeEditor extends StatefulWidget {
  final String initialCode;
  final ValueChanged<String>? onChanged;

  const VSCodeEditor({
    super.key,
    required this.initialCode,
    this.onChanged,
  });

  @override
  State<VSCodeEditor> createState() => _VSCodeEditorState();
}

class _VSCodeEditorState extends State<VSCodeEditor> {
  late final SyntaxTextEditingController _controller;
  final TreeSitterEnhanced _treeSitter = TreeSitterEnhanced.instance;
  final FocusNode _focusNode = FocusNode();
  final ScrollController _scrollController = ScrollController();
  final GlobalKey _editableTextKey = GlobalKey();

  // 选择状态
  bool _isDragging = false;
  TextPosition? _dragStartPosition;
  OverlayEntry? _toolbarOverlay;

  // 行高控制
  double _lineHeight = 1.3;

  @override
  void initState() {
    super.initState();
    _controller = SyntaxTextEditingController(
      text: widget.initialCode,
      treeSitter: _treeSitter,
    );
    _controller.addListener(_onTextChanged);
  }

  void _onTextChanged() {
    widget.onChanged?.call(_controller.text);
    setState(() {}); // 更新行号和调试信息
  }

  @override
  void dispose() {
    _controller.removeListener(_onTextChanged);
    _controller.dispose();
    _focusNode.dispose();
    _scrollController.dispose();
    _hideToolbar();
    super.dispose();
  }

  int get _lineCount => _controller.text.split('\n').length;

  // 构建行号，确保与文本行完全对齐
  Widget _buildLineNumbers() {
    final lines = _controller.text.split('\n');
    final lineNumbersText = lines.asMap().entries.map((entry) {
      return '${entry.key + 1}';
    }).join('\n');

    // 使用TextPainter精确测量行高
    final textPainter = TextPainter(
      text: TextSpan(
        text: '1\n2\n3', // 测量用的示例文本
        style: TextStyle(
          fontSize: 14,
          fontFamily: 'JetBrains Mono',
          height: _lineHeight,
        ),
      ),
      textDirection: TextDirection.ltr,
      maxLines: null,
    );
    textPainter.layout(maxWidth: 1000);

    final lineMetrics = textPainter.computeLineMetrics();
    final lineHeight = lineMetrics.isNotEmpty ? lineMetrics.first.height : 21.0;

    return Text(
      lineNumbersText,
      style: TextStyle(
        fontSize: 14,
        fontFamily: 'JetBrains Mono',
        color: const Color(0xFF858585),
        height: _lineHeight,
      ),
      textAlign: TextAlign.right,
      strutStyle: StrutStyle(
        fontSize: 14,
        forceStrutHeight: true,
        leading: 0.0,
        // 使用精确测量的行高
        height: _lineHeight,
      ),
    );
  }

  // 获取文本位置
  TextPosition? _getTextPositionFromOffset(Offset localOffset) {
    final RenderBox? renderBox =
        _editableTextKey.currentContext?.findRenderObject() as RenderBox?;
    if (renderBox == null) return null;

    final TextPainter textPainter = TextPainter(
      text: _controller.buildTextSpan(
        context: context,
        style: TextStyle(
          fontSize: 14,
          fontFamily: 'JetBrains Mono',
          height: _lineHeight,
        ),
        withComposing: false,
      ),
      textDirection: TextDirection.ltr,
    );

    textPainter.layout(maxWidth: renderBox.size.width);
    return textPainter.getPositionForOffset(localOffset);
  }

  // 开始拖动选择
  void _onPanStart(DragStartDetails details) {
    final position = _getTextPositionFromOffset(details.localPosition);
    if (position == null) return;

    setState(() {
      _isDragging = true;
      _dragStartPosition = position;
      _controller.selection = TextSelection.collapsed(offset: position.offset);
    });
    _hideToolbar();
  }

  // 拖动更新选择
  void _onPanUpdate(DragUpdateDetails details) {
    if (!_isDragging || _dragStartPosition == null) return;

    final position = _getTextPositionFromOffset(details.localPosition);
    if (position == null) return;

    setState(() {
      _controller.selection = TextSelection(
        baseOffset: _dragStartPosition!.offset,
        extentOffset: position.offset,
      );
    });
  }

  // 结束拖动选择
  void _onPanEnd(DragEndDetails details) {
    setState(() {
      _isDragging = false;
      _dragStartPosition = null;
    });
  }

  // 长按选中词
  void _onLongPress() {
    final selection = _controller.selection;
    if (selection.isCollapsed) {
      // 选中当前词
      final text = _controller.text;
      final offset = selection.baseOffset;

      // 找到词的边界
      int start = offset;
      int end = offset;

      // 向前找
      while (start > 0 && _isWordChar(text[start - 1])) {
        start--;
      }

      // 向后找
      while (end < text.length && _isWordChar(text[end])) {
        end++;
      }

      if (start != end) {
        setState(() {
          _controller.selection =
              TextSelection(baseOffset: start, extentOffset: end);
        });
      }
    }
  }

  bool _isWordChar(String char) {
    return RegExp(r'[a-zA-Z0-9_]').hasMatch(char);
  }

  // 显示选择工具栏
  void _showToolbar() {
    _hideToolbar();

    final RenderBox? renderBox =
        _editableTextKey.currentContext?.findRenderObject() as RenderBox?;
    if (renderBox == null || _controller.selection.isCollapsed) return;

    final selection = _controller.selection;
    final textPainter = TextPainter(
      text: _controller.buildTextSpan(
        context: context,
        style: TextStyle(
          fontSize: 14,
          fontFamily: 'JetBrains Mono',
          height: _lineHeight,
        ),
        withComposing: false,
      ),
      textDirection: TextDirection.ltr,
    );

    textPainter.layout(maxWidth: renderBox.size.width);
    final startOffset = textPainter.getOffsetForCaret(
      TextPosition(offset: selection.start),
      Rect.zero,
    );
    final endOffset = textPainter.getOffsetForCaret(
      TextPosition(offset: selection.end),
      Rect.zero,
    );

    final globalPosition = renderBox.localToGlobal(
      Offset((startOffset.dx + endOffset.dx) / 2, startOffset.dy - 40),
    );

    _toolbarOverlay = OverlayEntry(
      builder: (context) => Positioned(
        left: globalPosition.dx - 75,
        top: globalPosition.dy,
        child: Material(
          elevation: 8,
          borderRadius: BorderRadius.circular(8),
          color: const Color(0xFF2D2D30),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              _buildToolbarButton('Copy', Icons.copy, _copySelection),
              _buildToolbarButton('Select All', Icons.select_all, _selectAll),
            ],
          ),
        ),
      ),
    );

    Overlay.of(context).insert(_toolbarOverlay!);
  }

  Widget _buildToolbarButton(
      String label, IconData icon, VoidCallback onPressed) {
    return InkWell(
      onTap: () {
        onPressed();
        _hideToolbar();
      },
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 16, color: Colors.white),
            const SizedBox(width: 8),
            Text(label,
                style: const TextStyle(color: Colors.white, fontSize: 12)),
          ],
        ),
      ),
    );
  }

  // 隐藏工具栏
  void _hideToolbar() {
    _toolbarOverlay?.remove();
    _toolbarOverlay = null;
  }

  // 复制选中文本
  void _copySelection() {
    if (!_controller.selection.isCollapsed) {
      final selectedText = _controller.selection.textInside(_controller.text);
      Clipboard.setData(ClipboardData(text: selectedText));
    }
  }

  // 全选
  void _selectAll() {
    setState(() {
      _controller.selection = TextSelection(
        baseOffset: 0,
        extentOffset: _controller.text.length,
      );
    });
  }

  // 右键菜单
  void _onSecondaryTap(TapDownDetails details) {
    final position = _getTextPositionFromOffset(details.localPosition);
    if (position == null) return;

    // 如果点击位置不在选择范围内，移动光标到点击位置
    if (_controller.selection.isCollapsed ||
        position.offset < _controller.selection.start ||
        position.offset > _controller.selection.end) {
      setState(() {
        _controller.selection =
            TextSelection.collapsed(offset: position.offset);
      });
    }

    _showContextMenu(details.globalPosition);
  }

  void _showContextMenu(Offset globalPosition) {
    showMenu(
      context: context,
      position: RelativeRect.fromLTRB(
        globalPosition.dx,
        globalPosition.dy,
        globalPosition.dx + 1,
        globalPosition.dy + 1,
      ),
      items: [
        PopupMenuItem(
          value: 'copy',
          enabled: !_controller.selection.isCollapsed,
          child: const Row(
            children: [
              Icon(Icons.copy, size: 16),
              SizedBox(width: 8),
              Text('Copy'),
            ],
          ),
        ),
        PopupMenuItem(
          value: 'paste',
          child: const Row(
            children: [
              Icon(Icons.paste, size: 16),
              SizedBox(width: 8),
              Text('Paste'),
            ],
          ),
        ),
        PopupMenuItem(
          value: 'selectAll',
          child: const Row(
            children: [
              Icon(Icons.select_all, size: 16),
              SizedBox(width: 8),
              Text('Select All'),
            ],
          ),
        ),
      ],
    ).then((value) {
      switch (value) {
        case 'copy':
          _copySelection();
          break;
        case 'paste':
          _paste();
          break;
        case 'selectAll':
          _selectAll();
          break;
      }
    });
  }

  void _paste() async {
    final data = await Clipboard.getData('text/plain');
    if (data?.text != null) {
      final selection = _controller.selection;
      final newText = _controller.text.replaceRange(
        selection.start,
        selection.end,
        data!.text!,
      );

      setState(() {
        _controller.text = newText;
        _controller.selection = TextSelection.collapsed(
          offset: selection.start + data.text!.length,
        );
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      color: const Color(0xFF1E1E1E),
      child: Column(
        children: [
          // Debug info bar
          Container(
            padding: const EdgeInsets.all(8),
            color: const Color(0xFF252526),
            child: Row(
              children: [
                Text(
                  'Tokens: ${_controller.tokenCount}',
                  style: const TextStyle(color: Colors.grey, fontSize: 12),
                ),
                const SizedBox(width: 16),
                Text(
                  'Lines: $_lineCount',
                  style: const TextStyle(color: Colors.grey, fontSize: 12),
                ),
                const SizedBox(width: 16),
                Text(
                  'Cursor: ${_controller.selection.baseOffset}',
                  style: const TextStyle(color: Colors.grey, fontSize: 12),
                ),
                if (!_controller.selection.isCollapsed) ...[
                  const SizedBox(width: 16),
                  Text(
                    'Selected: ${_controller.selection.end - _controller.selection.start} chars',
                    style: const TextStyle(color: Colors.blue, fontSize: 12),
                  ),
                ],
                const Spacer(),
                // 行高控制
                Row(
                  children: [
                    const Text(
                      'Line Height: ',
                      style: TextStyle(color: Colors.grey, fontSize: 12),
                    ),
                    DropdownButton<double>(
                      value: _lineHeight,
                      dropdownColor: const Color(0xFF2D2D30),
                      style: const TextStyle(color: Colors.white, fontSize: 12),
                      underline: Container(),
                      items: [
                        1.0,
                        1.1,
                        1.2,
                        1.3,
                        1.4,
                        1.5,
                        1.6,
                        1.7,
                        1.8,
                        1.9,
                        2.0
                      ]
                          .map((height) => DropdownMenuItem<double>(
                                value: height,
                                child: Text('${height.toStringAsFixed(1)}'),
                              ))
                          .toList(),
                      onChanged: (value) {
                        if (value != null) {
                          setState(() {
                            _lineHeight = value;
                          });
                        }
                      },
                    ),
                  ],
                ),
              ],
            ),
          ),
          // Editor area with line numbers
          Expanded(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Line numbers column
                Container(
                  width: 60,
                  color: const Color(0xFF252526),
                  child: SingleChildScrollView(
                    controller: _scrollController,
                    child: Padding(
                      padding: const EdgeInsets.only(
                          top: 2.0, left: 8.0, right: 8.0, bottom: 16.0),
                      child: _buildLineNumbers(),
                    ),
                  ),
                ),
                // Vertical divider
                Container(
                  width: 1,
                  color: const Color(0xFF3E3E3E),
                ),
                // Editor column
                Expanded(
                  child: Theme(
                    data: Theme.of(context).copyWith(
                      textSelectionTheme: TextSelectionThemeData(
                        selectionColor:
                            const Color(0xFF3399FF).withOpacity(0.4),
                        cursorColor: const Color(0xFFFFFFFF),
                        selectionHandleColor: const Color(0xFF3399FF),
                      ),
                    ),
                    child: SingleChildScrollView(
                      controller: _scrollController,
                      child: ConstrainedBox(
                        constraints: BoxConstraints(
                          minHeight: MediaQuery.of(context).size.height - 200,
                        ),
                        child: GestureDetector(
                          onPanStart: _onPanStart,
                          onPanUpdate: _onPanUpdate,
                          onPanEnd: _onPanEnd,
                          onLongPress: _onLongPress,
                          onSecondaryTapDown: _onSecondaryTap,
                          onTap: () {
                            _focusNode.requestFocus();
                            _hideToolbar();
                          },
                          child: Padding(
                            padding: const EdgeInsets.only(
                                left: 4.0, // 减少左侧间距
                                right: 16.0,
                                top: 0.0, // 减少顶部间距，让文本向上移动
                                bottom: 16.0),
                            child: EditableText(
                              key: _editableTextKey,
                              controller: _controller,
                              focusNode: _focusNode,
                              style: TextStyle(
                                fontSize: 14,
                                fontFamily: 'JetBrains Mono',
                                color: const Color(0xFFD4D4D4),
                                height: _lineHeight,
                                backgroundColor: Colors.transparent,
                                decoration: TextDecoration.none,
                              ),
                              cursorColor: const Color(0xFFFFFFFF),
                              backgroundCursorColor: const Color(0xFF264F78),
                              cursorWidth: 2.0,
                              cursorRadius: const Radius.circular(1.0),
                              selectionColor:
                                  const Color(0xFF264F78).withOpacity(0.4),
                              maxLines: null,
                              textAlign: TextAlign.left,
                              textDirection: TextDirection.ltr,
                              autocorrect: false,
                              smartDashesType: SmartDashesType.disabled,
                              smartQuotesType: SmartQuotesType.disabled,
                              enableSuggestions: false,
                              keyboardType: TextInputType.multiline,
                              textInputAction: TextInputAction.newline,
                              showSelectionHandles: false, // 我们自己处理选择
                              strutStyle: StrutStyle(
                                fontSize: 14,
                                height: _lineHeight,
                                forceStrutHeight: true,
                                leading: 0.0, // 移除额外的行间距
                              ),
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class SyntaxTextEditingController extends TextEditingController {
  final TreeSitterEnhanced treeSitter;
  int _tokenCount = 0;

  SyntaxTextEditingController({
    required this.treeSitter,
    String? text,
  }) : super(text: text) {
    _updateTokenCount();
  }

  int get tokenCount => _tokenCount;

  void _updateTokenCount() {
    final tokens = treeSitter.highlight(text);
    _tokenCount = tokens.length;
  }

  @override
  set text(String newText) {
    super.text = newText;
    _updateTokenCount();
  }

  @override
  TextSpan buildTextSpan({
    required BuildContext context,
    TextStyle? style,
    required bool withComposing,
  }) {
    // 直接使用 Tree-sitter Enhanced 的 buildTextSpan 方法
    return treeSitter.buildTextSpan(text, baseStyle: style);
  }

  // 辅助方法：将Token位置转换为TextPosition
  TextPosition tokenStartToTextPosition(int tokenStart) {
    return TextPosition(offset: tokenStart);
  }

  TextPosition tokenEndToTextPosition(int tokenEnd) {
    return TextPosition(offset: tokenEnd);
  }

  // 辅助方法：选择特定Token
  void selectToken(int startOffset, int endOffset) {
    selection = TextSelection(
      baseOffset: startOffset,
      extentOffset: endOffset,
    );
  }

  // 辅助方法：获取当前光标所在行
  int getCurrentLine() {
    final beforeCursor = text.substring(0, selection.baseOffset);
    return beforeCursor.split('\n').length;
  }

  // 辅助方法：获取指定行的起始offset
  int getLineStartOffset(int lineNumber) {
    final lines = text.split('\n');
    if (lineNumber <= 0 || lineNumber > lines.length) return 0;

    int offset = 0;
    for (int i = 0; i < lineNumber - 1; i++) {
      offset += lines[i].length + 1; // +1 for '\n'
    }
    return offset;
  }
}
