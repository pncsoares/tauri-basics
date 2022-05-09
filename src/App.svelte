<script lang="ts">
	import { writeFile, FsTextFileOptions } from '@tauri-apps/api/fs';
	import { message } from '@tauri-apps/api/dialog';

	const handleClicks = async () => {
		const f: FsTextFileOptions = {
			path: './test.txt',
			contents: 'Cogito, ergo sum'
		}

		await writeFile(f);
		await message('file saved!');
	}
</script>

<main>
	<button on:click={handleClicks}>
		Create a file
	</button>
</main>
