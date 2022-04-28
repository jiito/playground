(if (eq (what-line) "Line 20")
    (princ "Im at line 20")
    (princ "Im not at line 20"))

(defun add-numbers (num1, num2)
  (+ num1 num2))

(add-numbers 3 5)

(search-forward "hello")
Some test text
Some more text
Hello
(goto-char 0)
Some end text


(defun there-and-back ()
  (save-excursion
    (previous-line)(insert "ABOVE")))

(there-and-back)


; defining an emacs interactive function
(defun name-func (name place)
  "Take a name and place from the user and print a string with them in the buffer"
  (interactive "MName:\nMPlace:")
  (save-excursion
    (goto-char (point-max))
    (insert "I am " name " from " place)))

I am Ben from Midd

; bind it to the keyboard
(global-set-key (kbd "C-h C-h") 'name-func)
