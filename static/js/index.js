$(async function () {
    let hotkey_to_element = {
        97: "number_card_1", // a
        115: "number_card_2", // s
        100: "number_card_3", // d
        102: "number_card_4", // f
        106: "operator_add", // j
        107: "operator_subtract", // k
        108: "operator_multiply", // l
        59: "operator_divide", // ;
        117: "special_operator_reset", // u
        105: "special_operator_next", // i
    };

    const log_element = $("#log");
    const number_cards = $("[id^=number_card]");
    const operator_cards = $("[id^=operator]");
    const special_operator_cards = $("[id^=special_operator]");

    const selected_class = "bg-blue-100";
    const hidden_class = "invisible";
    const win_class = "bg-yellow-500 scale-105 transition-all";

    let cards;
    let solutions;

    function is_num_clicked() {
        return number_cards
            .toArray()
            .some((e) => $(e).hasClass(selected_class));
    }

    function is_op_clicked() {
        return operator_cards
            .toArray()
            .some((e) => $(e).hasClass(selected_class));
    }

    function check_win() {
        let visible_cards = number_cards.filter(function () {
            return !$(this).hasClass(hidden_class);
        });

        if (
            visible_cards.length == 1 &&
            visible_cards.find("#card_value").attr("formula") == "24"
        ) {
            visible_cards.addClass(win_class);
            log_element.html(solutions.join("<br/>"));
            setTimeout(new_cardset, 500);
        }
    }

    function reset_cards() {
        number_cards.each(function () {
            $(this).removeClass(
                `${selected_class} ${hidden_class} ${win_class}`,
            );
        });
        operator_cards.each(function () {
            $(this).removeClass(selected_class);
        });

        set_html_cards(cards);
    }

    async function new_cardset() {
        let cardset = await fetch_cardset();
        cards = cardset.cards;
        solutions = cardset.solutions;

        reset_cards();
    }

    number_cards.click(function () {
        if (is_num_clicked() && is_op_clicked()) {
            let num_card_1 = number_cards.filter(function () {
                return $(this).hasClass(selected_class);
            });
            let num_card_2 = $(this);
            let op_card = operator_cards.filter(function () {
                return $(this).hasClass(selected_class);
            });

            if (num_card_1.attr("id") != num_card_2.attr("id")) {
                let num_1 = new Fraction(
                    num_card_1.find("#card_value").attr("formula"),
                );
                let num_2 = new Fraction(
                    num_card_2.find("#card_value").attr("formula"),
                );

                let result;
                switch (op_card.attr("op_value")) {
                    case "add":
                        result = num_1.add(num_2);
                        break;
                    case "subtract":
                        result = num_1.sub(num_2);
                        break;
                    case "multiply":
                        result = num_1.mul(num_2);
                        break;
                    case "divide":
                        result = num_1.div(num_2);
                        break;
                }

                // update with new values
                $(num_card_1)
                    .find("#card_value")
                    .attr("formula", result.toFraction());
                $(num_card_1).find("#card_value").html(result.toFraction());

                // hide 2nd card
                $(num_card_2).addClass(hidden_class);

                // deselect elements
                op_card.removeClass(selected_class);

                check_win();
            }
        } else {
            // toggle selection
            $(this).toggleClass(selected_class);
            number_cards.not(this).removeClass(selected_class);
        }
    });

    operator_cards.click(function () {
        if (is_num_clicked()) {
            $(this).toggleClass(selected_class);
            operator_cards.not(this).removeClass(selected_class);
        }
    });

    special_operator_cards.click(function () {
        let special_op = $(this).attr("op_value");

        if (special_op == "reset") {
            reset_cards();
        } else if (special_op == "next") {
            log_element.html(solutions.join("<br/>"));
            new_cardset();
        }
    });

    $(document).keypress(function (e) {
        for (let [hotkey, element] of Object.entries(hotkey_to_element)) {
            if (
                e.keyCode == hotkey &&
                !$(`#${element}`).hasClass(hidden_class)
            ) {
                $(`#${element}`).click();
            }
        }
    });

    async function fetch_cardset() {
        let response = await fetch("/cards");
        let json = await response.json();

        return json;
    }

    function set_html_cards(cards) {
        for (let [idx, card] of cards.entries()) {
            $(`#number_card_${idx + 1} > #card_value`).html(card);
            $(`#number_card_${idx + 1} > #card_value`).attr("formula", card);
        }
    }

    let cardset = await fetch_cardset();
    cards = cardset.cards;
    solutions = cardset.solutions;

    set_html_cards(cards);
});
