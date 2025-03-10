export type Conversation = {
	id: number;
	last_message_at: string;
	owner_id: number;
	status: string;
	title: string;
};

export const conversations = $state({
	value: [] as Conversation[]
});
