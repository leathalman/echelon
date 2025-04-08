export type Message = {
	content: string;
	role: 'User' | 'Assistant';
};

export const newMessage = $state({
	shouldStartCompletion: false,
	isAwaitingStream: false,
	isStreaming: false,
	value: {} as Message,
});
