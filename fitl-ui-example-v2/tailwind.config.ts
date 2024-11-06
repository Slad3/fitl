import aspectRatio from '@tailwindcss/aspect-ratio';
import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';

export default {
    content: [
        "./src/**/*.{html,js,svelte,ts}",
        "./node_modules/flowbite/**/*.js"
    ],

    plugins: [
        typography,
        forms,
        containerQueries,
        aspectRatio,
        // eslint-disable-next-line @typescript-eslint/no-require-imports
        require('flowbite/plugin')
    ],
    darkMode: 'media',
    theme: {
        extend: {
            colors: {
                'primary-background': "#212121",
                'secondary-background': "#111111",               
                // 'table-header': "#666666",                
                // 'table-header': "rgb(127 29 29)",              
                'table-header': "#336699",              
                'table-primary': "#333333",                
                'table-secondary': "#444444",
                // 'hover-accent': "rgb(54 83 20)"         
                // 'hover-accent': "rgb(127 29 29)"         
                'hover-accent': "#336699"         

            }
        },
        colors: {
            grey: "#212121"
        },
        fontFamily: {
            'body': [
                'Inter',
                'ui-sans-serif',
                'system-ui',
                '-apple-system',
                'system-ui',
                'Segoe UI',
                'Roboto',
                'Helvetica Neue',
                'Arial',
                'Noto Sans',
                'sans-serif',
                'Apple Color Emoji',
                'Segoe UI Emoji',
                'Segoe UI Symbol',
                'Noto Color Emoji'
            ],
            'sans': [
                'Inter',
                'ui-sans-serif',
                'system-ui',
                '-apple-system',
                'system-ui',
                'Segoe UI',
                'Roboto',
                'Helvetica Neue',
                'Arial',
                'Noto Sans',
                'sans-serif',
                'Apple Color Emoji',
                'Segoe UI Emoji',
                'Segoe UI Symbol',
                'Noto Color Emoji'
            ]
        }
    }
} satisfies Config;
