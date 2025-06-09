function fetch_with_timeout(url, options, timeout = 6000) {
    return Promise.race([
        fetch(url, options),
        new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), timeout)),
    ]);
}

let text = "print('Hello Python!');";

 const params = {
            lang:'python',
            code_block: text,
        };

let  result_block = {
            innerText:'',
            code_block: text,
        };

// console.log(params);

let data = JSON.stringify(params);

fetch_with_timeout('http://127.0.0.1:3333/api/v1/build-code', {
        headers: {
            'Content-Type': 'application/json',
        },
        method: 'POST',
        // mode: 'cors',
        body: data,
    })
        .then(response => response.json())
        .then(response => {
            if (response.result.trim() === '') {
                result_block.innerText = 'No output';
                // result_block.classList.add('result-no-output');
            } else {
                result_block.innerText = response.result;
                // result_block.classList.remove('result-no-output');
                console.log(result_block.innerText);
                console.log(result_block);
            }
        })
        .catch(error => {
            console.error('Error:', error);
            result_block.innerText = 'Error: ' + error.message;
            // result_block.classList.add('result-error');
        });