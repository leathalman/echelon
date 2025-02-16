import Root from './textarea.svelte';
import Plain from './textarea-plain.svelte';

type FormTextareaEvent<T extends Event = Event> = T & {
	currentTarget: EventTarget & HTMLTextAreaElement;
};

type TextareaEvents = {
	blur: FormTextareaEvent<FocusEvent>;
	change: FormTextareaEvent<Event>;
	click: FormTextareaEvent<MouseEvent>;
	focus: FormTextareaEvent<FocusEvent>;
	keydown: FormTextareaEvent<KeyboardEvent>;
	keypress: FormTextareaEvent<KeyboardEvent>;
	keyup: FormTextareaEvent<KeyboardEvent>;
	mouseover: FormTextareaEvent<MouseEvent>;
	mouseenter: FormTextareaEvent<MouseEvent>;
	mouseleave: FormTextareaEvent<MouseEvent>;
	paste: FormTextareaEvent<ClipboardEvent>;
	input: FormTextareaEvent<InputEvent>;
};

export {
	Root,
	Plain as TextareaPlain,
	//
	Root as Textarea,
	type TextareaEvents,
	type FormTextareaEvent
};
