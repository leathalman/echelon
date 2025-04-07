export type Message = {
	content: string;
	role: 'User' | 'Assistant';
};

export const messages = $state({
	value: [] as Message[]
});

export const newMessage = $state({
	shouldStartCompletion: false,
	isAwaitingStream: false,
	isStreaming: false,
});
