import { RouteObject } from 'react-router-dom';
import { CreateGame } from './create-game.component';

export const createGameRoutes: RouteObject[] = [
    {
        path: "/create",
        element: <CreateGame />
    }
]