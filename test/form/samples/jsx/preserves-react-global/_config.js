module.exports = defineTest({
	solo: true, //x,
	description: 'preserves React variable when preserving JSX output',
	options: {
		jsx: {
			factory: 'React.createElement',
			fragmentFactory: 'React.Fragment',
			mode: 'preserve'
		}
	}
});
