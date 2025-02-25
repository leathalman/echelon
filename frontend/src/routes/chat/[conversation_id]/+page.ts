// +page.ts (for client-side loading)
export async function load({ params, fetch }) {
	// depends(`chat:${params.conversation_id}`);

	const conversation_id = params.conversation_id;

	// how to get ID from URL?? this is not the way...

	console.log(conversation_id);
	// try {
	// 	const response = await fetch(
	// 		`http://localhost:8000/api/conversations/${conversation_id}/messages`
	// 	);
	// 	const messages = await response.json();
	//
	// 	return {
	// 		chat_id: conversation_id,
	// 		messages: messages
	// 	};
	// } catch (error) {
	// 	console.error(`Failed to fetch conversations: ${error}`);
	// 	return {
	// 		chat_id: conversation_id,
	// 		messages: []
	// 	};
	// }
}
