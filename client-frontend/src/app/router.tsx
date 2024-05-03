import { createBrowserRouter } from 'react-router-dom';
import { Layout } from './layout.component';

import { landingRoutes } from 'libs/landing';
import { createGameRoutes } from 'libs/create-game';

export const memoryRouter = createBrowserRouter([
    {
        path: "/",
        element: <Layout />,
        children: [
            ...landingRoutes,
            ...createGameRoutes,
        ]
    }
]);

