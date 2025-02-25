export interface Conversation {
	id: number;
	last_message_at: string;
	owner_id: number;
	status: string;
	title: string;
}

export class ConversationsResponse {
	conversations: Conversation[];

	constructor(data: any) {
		this.conversations = data.conversations.map((conv: any) => {
			return {
				id: conv.id,
				last_message_at: conv.last_message_at,
				owner_id: conv.owner_id,
				status: conv.status,
				title: conv.title
			};
		});
	}

	getActiveConversations(): Conversation[] {
		return this.conversations.filter((conv) => conv.status === 'Active');
	}

	getConversationsSortedByDate(): Conversation[] {
		return [...this.conversations].sort((a, b) => {
			return new Date(b.last_message_at).getTime() - new Date(a.last_message_at).getTime();
		});
	}

	get count(): number {
		return this.conversations.length;
	}
}

export default ConversationsResponse;
