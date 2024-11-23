$(async function () {


    async function fetch_cardset() {
        let response = await fetch("/cards");
        let json = await response.json();

        return json;
    }

    function set_html_cards(cards) {
        for (let [idx, card] of cards.entries()) {
            $(`#number_card_${idx}>.value`).html(card);
        }
    }

    let cardset = await fetch_cardset();
    set_html_cards(cardset.cards);


});