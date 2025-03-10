export type Message = {
	content: string;
	role: 'User' | 'Assistant';
};

export const messages = $state({
	value: [] as Message[]
});

export const newMessage = $state({
	content: '',
	completionPending: false,
	completion: ''
});
