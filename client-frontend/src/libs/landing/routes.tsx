import { RouteObject } from 'react-router-dom';
import { Landing } from './landing.component';

export const landingRoutes: RouteObject[] = [
    {
        index: true,
        element: <Landing />
    }
]