const js = import("./bonus/bonus.js");

js.then(js => {
    const state = {
        base: 0,
        values: {}
    };

    const getValues = () => {
        const values = [];
        for (const key in state.values) {
            if (state.values.hasOwnProperty(key)) {
                const value = state.values[key];
                values.push(value);                
            }
        }
        return new Float64Array(values);
    }

    const getIncome = () => {
        const element = document.getElementById("income");
        const parsed = element.value.replace(/[^\d\.]/g, '');
        return Number(parsed);
    }

    window.register = (id) => {
        const element = document.getElementById(id);
        element.addEventListener('input', e => {
            const value = Number(e.target.value);
            const input = e.target.id;

            state.values[id] = value;
            js.handle(input, value);
            js.calculate(state.base, getValues(), getIncome())
        })
    }

    const income = document.getElementById("income");
    income.addEventListener('input', e => {
        const value = getIncome();
        if (value) {
            js.calculate(state.base, getValues(), value)
        }
    })

    js.run();
    state.base = js.get_base_value();    
});