	//examine the document object

	//console.dir(document);
	let titles = document.getElementsByClassName('title');
	for(let i= 0; i<titles.length; i++){
		console.log(titles[i]);
	}
	let lis = document.getElementsByTagName('li');

	for(let i in lis){
		console.log(lis[i]);
	}

	let arr = document.getElementsByClassName('title');
	Array.from(arr).forEach(function(item){
		console.log(item);
	});


	const wmf = document.querySelector('#book-list li:nth-child(2) .name');
	console.log(wmf);
	let books = document.querySelectorAll('#book-list li .name'); // grab all lists\
	console.log(books);

	const link = document.querySelector('#page-banner a');

	link.addEventListener('click', function(e){
		e.preventDefault();
	});


	//delete books by clicking delete button
	const list = document.querySelector('#book-list ul');
	list.addEventListener('click', function(e){
		const li = e.target.parentElement;
		list.removeChild(li);
	});

    //add book-list
    const addForm = document.forms['add-book'];
    addForm.addEventListener('submit', function(e){
	e.preventDefault();
	const value = addForm.querySelector('input[type="text"]').value;

	//create elements
	const li = document.createElement('li');
	const bookName = document.createElement('span');
	const deleteBtn = document.createElement('span');

	//append to document
	li.appendChild(bookName);
	li.appendChild(deleteBtn);

	//add content
    bookName.textContent = value;
    deleteBtn.textContent = 'delete';


    //add classes
    bookName.classList.add('name');
    deleteBtn.classList.add('delete');

	list.appendChild(li);
	});