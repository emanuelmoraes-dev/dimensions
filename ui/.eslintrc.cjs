module.exports = {
    'env': {
        'browser': true,
        'es2021': true
    },
    'extends': [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended'
    ],
    'overrides': [
        {
            'env': {
                'node': true
            },
            'files': [
                '.eslintrc.{js,cjs}'
            ],
            'parserOptions': {
                'sourceType': 'script'
            }
        }
    ],
    'parser': '@typescript-eslint/parser',
    'parserOptions': {
        'ecmaVersion': 'latest',
        'sourceType': 'module'
    },
    'plugins': [
        '@typescript-eslint'
    ],
    'rules': {
        'indent': ['error', 4], // 4 spaces ident
        'linebreak-style': ['error', 'unix'], // lf
        'semi': ['error','never'], // no semicolon
        'quotes': ['error','single'], // single quotes
        'object-curly-spacing': ['error', 'never', {'arraysInObjects': false, 'objectsInObjects': false}], // no spaces in braces
        'no-unused-vars': 'off',
        '@typescript-eslint/no-unused-vars': ['error', {'args': 'none'}],
        '@typescript-eslint/no-explicit-any': ['off']
    }
}
