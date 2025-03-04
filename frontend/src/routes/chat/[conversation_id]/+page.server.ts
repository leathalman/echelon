export async function load({ parent }) {
	const parentData = await parent();
	console.log(parentData);
	// comes up empty?
	return {
		...parentData
	};
}
