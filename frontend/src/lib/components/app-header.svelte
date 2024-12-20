<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';

	export let type: string = 'Student';
	export let applicationTitle: String = 'EchelonGPT';
</script>

<div class="flex flex-col">
	<div class="flex flex-row flex-growth justify-between">
		<div class="flex flex-col justify-center">
			<div class="text-xs font-thin">{type}</div>
			<div class="text-lg font-bold text-neutral-100 leading-5">{applicationTitle}</div>
		</div>
		<Avatar.Root class="w-9 h-9">
			<Avatar.Image src="https://github.com/shadcn.png" alt="@shadcn" />
			<Avatar.Fallback>HL</Avatar.Fallback>
		</Avatar.Root>
	</div>
</div>