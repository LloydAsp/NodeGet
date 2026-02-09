export const themeEnConfig = {
    nav: [
        { text: 'Index', link: '/en/wip' },
        { text: 'Quick Start', link: '/en/wip' },
        { text: 'Configuration Guide', link: '/en/wip' },
        { text: 'API', link: '/en/wip' },
    ],
    sidebar: [
        {
            text: 'Configuration Guide',
            items: [
                { text: 'Overview', link: '/en/guide/config/index.md' },
                { text: 'Server Configuration', link: '/en/guide/config/server.md' },
                { text: 'Agent Configuration', link: '/en/guide/config/agent.md' }
            ]
        },
        {
            text: 'API Documentation',
            items: [
                { text: 'Overview', link: '/en/api/index.md' },
                { text: 'Error Handling', link: '/en/api/errors.md' }
            ]
        }
    ],
}