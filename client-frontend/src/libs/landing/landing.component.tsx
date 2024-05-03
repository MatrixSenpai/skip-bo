import { FunctionComponent } from 'react';
import { Link } from 'react-router-dom';

export const Landing: FunctionComponent = () => {
    return (
        <div className='container py-2'>
            <div className='row mb-2'>
                <div className='col-12 text-center'>
                    <h1>Skip-Bo</h1>
                </div>
            </div>

            <div className='row'>
                <div className='col-12 col-lg-6'>
                    <div className='card'>
                        <div className='card-body'>
                            <div className='nav flex-column gap-2'>
                                <Link
                                    to='/create'
                                    className='btn btn-block btn-primary'
                                >
                                    Create new game
                                </Link>
                                <button
                                    disabled
                                    className='btn btn-block btn-disabled'
                                >
                                    Join Private Game
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div className='col-12 col-lg-6'>
                    <ul className='list-group'>
                        <li className='list-group-item list-group-item-light'>
                            Public Lobbies
                        </li>
                        <li className='list-group-item'>X Game</li>
                        <li className='list-group-item'>Y Game</li>
                        <li className='list-group-item'>Z Game</li>
                        <li className='list-group-item'>XX Game</li>
                    </ul>
                </div>
            </div>
        </div>
    );
};
