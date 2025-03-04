export type NewChatParams = {
	initialMessage: string;
	completionPending: boolean;
	completionResult: string;
	completionError: string;
};

export const newChatParams: NewChatParams = $state({
	initialMessage: '',
	completionPending: false,
	completionResult: '',
	completionError: ''
});
