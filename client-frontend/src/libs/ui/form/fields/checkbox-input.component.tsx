import { FunctionComponent, ReactElement } from 'react';
import { Field } from 'formik';

interface Props {
    name: string;
    label: string | ReactElement;
    isSwitch?: boolean;
    disabled?: boolean;
}

export const CheckboxInput: FunctionComponent<Props> = ({
    name,
    label,
    isSwitch,
    disabled,
}) => {
    return (
        <div className={`mb-3 form-check ${isSwitch && 'form-switch'}`}>
            <Field
                className='form-check-input'
                type='checkbox'
                name={name}
                disabled={disabled}
            />
            <label className='form-check-label'>{label}</label>
        </div>
    );
};
