/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                // Deep Night Cyan Neon & Cyber Purple Palette
                'void': '#010103', // Deep Black/Blue Void
                'obsidian': '#030308', // Surface Background
                'glass': 'rgba(11, 11, 15, 0.6)', // Glass Base

                // Primary: Electric Cyan
                'primary': {
                    DEFAULT: '#00f0ff',
                    dim: 'rgba(0, 240, 255, 0.1)',
                    glow: 'rgba(0, 240, 255, 0.6)',
                    50: '#f0fdfa',
                    100: '#ccfbf1',
                    200: '#99f6e4',
                    300: '#5eead4',
                    400: '#2dd4bf',
                    500: '#00f0ff', // Main Cyan
                    600: '#0d9488',
                    700: '#0f766e',
                    800: '#115e59',
                    900: '#134e4a',
                },

                // Secondary: Cyber Purple
                'purple': {
                    DEFAULT: '#bf00ff',
                    dim: 'rgba(191, 0, 255, 0.1)',
                    glow: 'rgba(191, 0, 255, 0.6)',
                },

                // Functional Colors
                'emerald': {
                    DEFAULT: '#00ff9d', // Neon Mint
                    glow: 'rgba(0, 255, 157, 0.5)',
                },
                'warning': '#fbff00', // High-Voltage Yellow
                'error': '#ff003c',   // Cyber Red

                // Legacy mapping for compatibility
                'neon': '#00f0ff',
                'cyber': '#bf00ff',
            },
            fontFamily: {
                sans: ['Inter', 'var(--font-sans)', 'system-ui', 'sans-serif'],
                mono: ['JetBrains Mono', 'var(--font-mono)', 'Consolas', 'monospace'],
                display: ['Inter', 'var(--font-sans)', 'system-ui', 'sans-serif'],
            },
            backdropBlur: {
                xs: '2px',
                '2xl': '40px',
                '3xl': '64px',
            },
            borderRadius: {
                '4xl': '2rem',
            },
            boxShadow: {
                'glow': '0 0 20px rgba(139, 92, 246, 0.3)',
                'glow-lg': '0 0 40px rgba(139, 92, 246, 0.4)',
                'neon': '0 0 15px rgba(0, 240, 255, 0.5)',
                'cyber': '0 0 20px rgba(188, 0, 255, 0.3)',
                'cyan-glow': '0 0 15px rgba(6, 182, 212, 0.5)',
                'inner-glow': 'inset 0 0 20px rgba(6, 182, 212, 0.2)',
            },
            animation: {
                'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                'spin-slow': 'spin 3s linear infinite',
                'blob-1': 'blob 25s infinite alternate',
                'blob-2': 'blob 30s infinite alternate',
                'blob-3': 'blob 35s infinite alternate',
                'ping-slow': 'ping 3s cubic-bezier(0, 0, 0.2, 1) infinite',
                'ping-slower': 'ping 4s cubic-bezier(0, 0, 0.2, 1) infinite',
                'slide-up': 'slideUp 0.5s ease-out',
                'pulse-glow': 'pulseGlow 4s ease-in-out infinite',
                'shimmer': 'shimmer 2.5s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                'float': 'float 6s ease-in-out infinite',
                'glitch': 'glitch 1s linear infinite',
                'entrance-up': 'entranceUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) backwards',
            },
            keyframes: {
                blob: {
                    '0%': { transform: 'translate(0px, 0px) scale(1)' },
                    '33%': { transform: 'translate(30px, -50px) scale(1.1)' },
                    '66%': { transform: 'translate(-20px, 20px) scale(0.9)' },
                    '100%': { transform: 'translate(0px, 0px) scale(1)' },
                },
                slideUp: {
                    '0%': { transform: 'translateY(20px)', opacity: '0' },
                    '100%': { transform: 'translateY(0)', opacity: '1' },
                },
                pulseGlow: {
                    '0%, 100%': { opacity: '0.5', boxShadow: '0 0 20px rgba(34, 211, 238, 0.3)' },
                    '50%': { opacity: '1', boxShadow: '0 0 40px rgba(34, 211, 238, 0.6)' },
                },
                shimmer: {
                    '0%': { transform: 'translateX(-100%)' },
                    '100%': { transform: 'translateX(100%)' },
                },
                float: {
                    '0%, 100%': { transform: 'translateY(0)' },
                    '50%': { transform: 'translateY(-10px)' },
                },
                glitch: {
                    '2%, 64%': { transform: 'translate(2px,0) skew(0deg)' },
                    '4%, 60%': { transform: 'translate(-2px,0) skew(0deg)' },
                    '62%': { transform: 'translate(0,0) skew(5deg)' },
                },
                entranceUp: {
                    '0%': { opacity: '0', transform: 'translateY(20px)' },
                    '100%': { opacity: '1', transform: 'translateY(0)' },
                },
            },
        },
    },
    plugins: [],
}
