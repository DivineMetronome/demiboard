module.exports = {
  env: {
    browser: true,
    node: true,
  },
  plugins: ['@typescript-eslint'],
  parser: 'vue-eslint-parser',
  extends: [
    'eslint:recommended',
    'plugin:vue/vue3-recommended',
    'plugin:@typescript-eslint/eslint-recommended',
    'plugin:@typescript-eslint/recommended',
    'prettier',
    'prettier/vue',
  ],
  parserOptions: {
    parser: '@typescript-eslint/parser',
  },
  rules: {},
};
