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
        return document.getElementById("income").value;
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
        if (e.target.value) {
            js.calculate(state.base, getValues(), e.target.value);
        }
    })

    js.run();
    state.base = js.get_base_value();    
});