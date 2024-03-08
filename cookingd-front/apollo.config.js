module.exports = {
    client: {
      service: {
        name: 'cookingd',
        url: import.meta.env.VITE_BACKEND_URL,
      },
      includes: [
        'src/**/*.vue',
        'src/**/*.js',
      ],
    },
  }