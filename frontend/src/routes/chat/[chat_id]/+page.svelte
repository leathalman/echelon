<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';

	let question = 'How should I plan my next semester?';
	let contents =
		'## **Computer Science Major Requirements Progress**\n' +
		'To effectively plan your next semester, let’s consider your progress in your major (Computer Science), minor (Mathematics), and Honors College curriculum. Here\'s an analysis based on your transcript and program requirements:\n' +
		'✅ **Completed Core Courses:**\n' +
		'- COSC10403: Introduction to Programming (via AP credits).\n' +
		'- COSC20203: Techniques in Programming.\n' +
		'- COSC20803: Data Structures.\n' +
		'- COSC30203: Computer System Fundamentals.\n' +
		'- COSC30253: Computer Organization.\n' +
		'- COSC30403: Programming Language Concepts.\n' +
		'- COSC30603: Database Systems.\n' +
		'- COSC40203: Operating Systems.\n' +
		'- COSC40403: Analysis of Algorithms.\n' +
		'- CITE30103: Unix/Linux System Administration.\n' +

		'🔲 **Remaining Core Courses:**\n' +
		'- COSC40943: Software Engineering (currently enrolled).\n' +
		'- COSC40993: Senior Design Project (should be taken next semester to graduate on time).\n' +

		'🔲 **Electives:**\n' +
		'You need 12 credit hours of electives from CITE or 30000+ level COSC courses. You have completed:\n' +
		'- COSC40503: Artificial Intelligence.\n' +
		'- COSC40453: Object-Oriented Programming and Design Patterns.\n' +
		'- CITE30363: Web Technologies.\n' +

		'**Plan to take one more elective next semester to fulfill this requirement.**\n' +
		'---\n' +
		'## **Mathematics Minor Progress**\n' +
		'✅ **Completed Core Courses:**\n' +
		'- MATH10524: Calculus I (via AP credits).\n' +
		'- MATH20123: Discrete Mathematics I.\n' +
		'- MATH30123: Discrete Mathematics II.\n' +
		'- MATH10043: Elementary Statistics.\n' +
		'\n' +
		'🔲 **Remaining Core Courses:**\n' +
		'- MATH20524: Calculus II (required for minor).\n' +
		'- MATH30224: Linear Algebra (currently enrolled).\n' +
		'\n' +
		'**You need one more upper-level MATH course to meet the 18-hour requirement. Consider taking a 30000+ level elective such as MATH30853: Statistics or MATH30863: Differential Equations.**\n' +
		'\n' +
		'---\n' +
		'\n' +
		'## **Honors College Requirements**\n' +
		'### **Lower-Division Honors:**\n' +
		'✅ **Completed:**\n' +
		'- Cultural Visions: Fulfilled via HNRS20243 and HNRS20113.\n' +
		'- Honors Electives: Fulfilled with honors courses, including HNRS20243 and PHIL10103.\n' +
		'\n' +
		'### **Upper-Division Honors:**\n' +
		'You are pursuing **University Honors**.  \n' +
		'🔲 **Remaining Requirements:**\n' +
		'- 9 credit hours of colloquia. You haven’t taken any University Honors colloquia yet. Consider courses like:\n' +
		'  - **HCOL41013**: Disease, Disorder, and Disability.\n' +
		'  - **HCOL41023**: Empathy.\n' +
		'\n' +
		'---\n' +
		'\n' +
		'## **Recommended Schedule for Next Semester**\n' +
		'### **1. Core Computer Science:**\n' +
		'- **COSC40993**: Senior Design Project (3 credits). This is critical to graduate on time.\n' +
		'\n' +
		'### **2. Mathematics Minor:**\n' +
		'- **MATH20524**: Calculus II (4 credits). A required course for your minor.\n' +
		'\n' +
		'### **3. Honors Colloquia:**\n' +
		'- Choose 1-2 University Honors courses (6 credits). Examples:\n' +
		'  - **HCOL41023**: Empathy.\n' +
		'  - **HCOL41473**: Superheroes and Culture.\n' +
		'\n' +
		'### **4. CS Elective or Additional Requirement:**\n' +
		'- **CITE40000+ or COSC30000+ Elective** (3 credits). Consider advanced topics like cybersecurity, networking, or machine learning.\n' +
		'\n' +
		'---\n' +
		'\n' +
		'### **Example Schedule**\n' +
		'| **Course**                        | **Credits** |\n' +
		'|------\n';

	marked.parse(
		contents.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, '')
	);

	let markdownWidth = $state();
	let textAreaHeight = $state(25);
</script>

<div class="flex flex-col items-center pt-24">
	<div bind:clientWidth={markdownWidth} style="margin-bottom: {textAreaHeight + 80}px"
			 class="flex flex-col w-[90%] md:max-w-156 space-y-2">
		<span class="text-sm text-gray-600">what are the objectives of the holiday party?</span>
		<div class="prose prose-sm">
			{@html marked(contents)}
		</div>
	</div>
	<div style="width: {markdownWidth}px" class="fixed bottom-0 pb-6 bg-background">
		<div class="flex flex-row bg-background rounded-lg shadow-lg border justify-between">
			<div class="flex items-center mx-1 px-2 my-2 w-[90%]">
				<TextareaPlain
					bind:height={textAreaHeight}
					class="w-full font-medium"
					placeholder="What else would you like to know?" />
			</div>
			<div class="flex items-end">
				<Button class="w-8 h-8 my-2 mx-2">
					<ArrowRight />
				</Button>
			</div>
		</div>
	</div>
</div>
