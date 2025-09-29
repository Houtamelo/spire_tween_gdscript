class_name SpireMisc

## AcceptDialog

static func do_dialog_text(node: AcceptDialog, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(AcceptDialogTweener.do_dialog_text(node, to, duration))

static func do_ok_button_text(node: AcceptDialog, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(AcceptDialogTweener.do_ok_button_text(node, to, duration))

## ConfirmationDialog

static func do_cancel_button_text(node: ConfirmationDialog, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(ConfirmationDialogTweener.do_cancel_button_text(node, to, duration))

## StatusIndicator

static func do_tooltip(node: StatusIndicator, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(StatusIndicatorTweener.do_tooltip(node, to, duration))

