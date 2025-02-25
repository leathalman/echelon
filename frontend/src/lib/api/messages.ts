export type Message = {
	conversation_id: number;
	content: string;
	role: 'User' | 'Assistant';
};
