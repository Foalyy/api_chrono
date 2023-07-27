let localTimestampDiff = 0;
let currentState = undefined;
let evtSource = undefined;


function millis() {
    return Date.now();
}

function timestamp() {
    return Math.round(millis() / 1000);
}

function serverTimestamp() {
    return timestamp() - localTimestampDiff;
}

function computeClockDiff() {
    let request = new XMLHttpRequest();
    request.onreadystatechange = function() {
        if (this.readyState == 4) {
            if (this.status == 200) {
                let serverTimestamp = JSON.parse(this.response);
                let localTimestamp = timestamp();
                localTimestampDiff = localTimestamp - serverTimestamp;
            }
        }
    };
    request.timeout = 3000;
    request.open('GET', getTimeURL, true);
    request.send();
}

function updateCurrentTime() {
    let currentServerTime = new Date(serverTimestamp() * 1000);
    $('.current-time').text(currentServerTime.toLocaleTimeString('fr-FR'));
}

function update() {
    request = new XMLHttpRequest();
    request.onreadystatechange = function() {
        if (this.readyState == 4) {
            if (this.status == 200) {
                state = JSON.parse(this.response);
                updateState(state);
                setConnectionState(true);
            }
        }
    };
    request.ontimeout = function(e) {
        setConnectionState(false);
    }
    request.timeout = 5000;
    request.open('GET', loadStateURL, true);
    request.send();
}

function updateState(newState) {
    // Countdown and Next launch
    if (newState.next_launch) {
        if (newState.next_launch.countdown_paused) {
            $('.countdown').addClass('countdown-paused');
        } else {
            $('.countdown').removeClass('countdown-paused');
        }
        let countdown = updateCountdown('.countdown-value', newState.next_launch.last_update, newState.next_launch.countdown, newState.next_launch.countdown_paused);
        setVisible('.countdown-value', newState.next_launch.countdown != null);
        setMainClock(countdown);
        $('.next-launch-empty').addClass('hidden');
        setMatricule('.next-launch-matricule', newState.next_launch.code);
        $('.next-launch-project-name').text(newState.next_launch.name);
        $('.next-launch-club').text(newState.next_launch.club_name);
        $('.next-launch-launchpad').text(newState.next_launch.launchpad_name);
        $('.next-launch-rocket-color').text(newState.next_launch.rocket_color);
        $('.next-launch-parachute-color').text(newState.next_launch.parachute_color);
        $('.next-launch-info').removeClass('hidden');
    } else {
        $('.countdown').removeClass('countdown-paused');
        $('.countdown-value').empty()
        $('.next-launch-info').addClass('hidden');
        setMainClock(0);
        removeMatricule('.next-launch-matricule');
        $('.next-launch-project-name').text('');
        $('.next-launch-club').text('');
        $('.next-launch-launchpad').text('');
        $('.next-launch-rocket-color').text('');
        $('.next-launch-parachute-color').text('');
        $('.next-launch-empty').removeClass('hidden');
    }

    // Safety lights
    updateLight('weather', newState.safety_checks.weather);
    updateLight('zone-safety', newState.safety_checks.zone_safety);
    updateLight('zone-fx', newState.zones.fx);
    updateLight('zone-mf', newState.zones.mf);

    // Launchpads
    $('.launchpad').each(function(launchpad) {
        let launchpadType = $(launchpad).data('launchpadType');
        let launchpadName = $(launchpad).data('launchpadName');
        let launchpadState = newState['launchpads_' + launchpadType][launchpadName];
        if (launchpadState == null) {
            $('.launchpad-' + launchpadName + '-info').addClass('hidden');
            $('.launchpad-' + launchpadName + '-empty').removeClass('hidden');
        } else {
            updateCountdown('.launchpad-' + launchpadName + '-countdown-value', launchpadState.last_update, launchpadState.countdown, launchpadState.countdown_paused);
            setVisible('.launchpad-' + launchpadName + '-countdown', launchpadState.countdown != null);
            setClassIf('.launchpad-' + launchpadName + '-countdown', 'launchpad-countdown-paused', launchpadState.countdown_paused);
            setMatricule('.launchpad-' + launchpadName + '-matricule', launchpadState.code);
            $('.launchpad-' + launchpadName + '-project-name').text(launchpadState.name);
            $('.launchpad-' + launchpadName + '-club').text(launchpadState.club_name);
            $('.launchpad-' + launchpadName + '-motor').text(launchpadState.motor_type);
            $('.launchpad-' + launchpadName + '-rocket-color').text(launchpadState.rocket_color);
            $('.launchpad-' + launchpadName + '-parachute-color').text(launchpadState.parachute_color);
            setVisible('.launchpad-' + launchpadName + '-telem', launchpadState.rf_onboard)
            $('.launchpad-' + launchpadName + '-telem-type').text(launchpadState.rf);
            $('.launchpad-' + launchpadName + '-telem-frequency').text(launchpadState.rf_frequency);
            setVisible('.launchpad-' + launchpadName + '-telem-on', launchpadState.rf_on);
            setVisible('.launchpad-' + launchpadName + '-spec-special', launchpadState.special_project);
            setVisible('.launchpad-' + launchpadName + '-spec-roll-control', launchpadState.roll_control);
            setVisible('.launchpad-' + launchpadName + '-spec-supersonic', launchpadState.supersonic);
            setVisible('.launchpad-' + launchpadName + '-spec-two-stage', launchpadState.two_stage);
            setVisible('.launchpad-' + launchpadName + '-project-specs', launchpadState.special_project || launchpadState.roll_control || launchpadState.supersonic || launchpadState.two_stage);
            setClassIf('.launchpad-' + launchpadName + '-motor-destocked', 'launchpad-check-ok', launchpadState.motor_destocked);
            setVisible('.launchpad-' + launchpadName + '-motor-destocked', !launchpadState.motor_inserted && !launchpadState.motor_wired);
            setClassIf('.launchpad-' + launchpadName + '-motor-inserted', 'launchpad-check-ok', launchpadState.motor_inserted);
            setVisible('.launchpad-' + launchpadName + '-motor-inserted', launchpadState.motor_inserted && !launchpadState.motor_wired);
            setClassIf('.launchpad-' + launchpadName + '-motor-wired', 'launchpad-check-ok', launchpadState.motor_wired);
            setVisible('.launchpad-' + launchpadName + '-motor-wired', launchpadState.motor_wired);
            setClassIf('.launchpad-' + launchpadName + '-pyro-ok', 'launchpad-check-ok', launchpadState.pyro_ok);
            setClassIf('.launchpad-' + launchpadName + '-club-ok', 'launchpad-check-ok', launchpadState.club_ok);
            setClassIf('.launchpad-' + launchpadName + '-ddo-ok', 'launchpad-check-ok', launchpadState.ddo_ok);
            setClassIf('.launchpad-' + launchpadName + '-rso-ok', 'launchpad-check-ok', launchpadState.rso_ok);
            setClassIf('.launchpad-' + launchpadName + '-landed', 'launchpad-check-ok', launchpadState.landed);
            setVisible('.launchpad-' + launchpadName + '-flight-result-nominal', launchpadState.flight_result == 'nominal')
            setClassIf('.launchpad-' + launchpadName + '-flight-result', 'status-light-green', launchpadState.flight_result == 'nominal');
            setVisible('.launchpad-' + launchpadName + '-flight-result-balistic', launchpadState.flight_result == 'balistic')
            setClassIf('.launchpad-' + launchpadName + '-flight-result', 'status-light-red', launchpadState.flight_result == 'balistic');
            setVisible('.launchpad-' + launchpadName + '-flight-result', launchpadState.flight_result == 'nominal' || launchpadState.flight_result == 'balistic')
            $('.launchpad-' + launchpadName + '-empty').addClass('hidden');
            $('.launchpad-' + launchpadName + '-info').removeClass('hidden');
        }
    });

    for (let place of ['clubs-tent', 'jupiter']) {
        let projects = newState[place.replace('-', '_')];
        let container = $('.' + place + '-info');
        container.empty();
        for (let project of projects) {
            let el = $('<div class="' + place + '-project"><span class="' + place + '-project-matricule matricule matricule-' + project.code.substring(0, 2).toLowerCase() + '">' + project.code + '</span> <span class="' + place + '-project-name">' + project.name + '</span> / <span class="' + place + '-project-club">' + project.club_name + '</span></div>');
            container.append(el);
        }
    }

    if (newState.message) {
        $('.banner-text').text(newState.message);
        $('.banner').removeClass('hidden');
    } else {
        $('.banner').addClass('hidden');
        $('.banner-text').text('');
    }

    currentState = newState;
}

function updateLight(light, color) {
    if (!$('.status-' + light).hasClass('status-light-' + color)) {
        $('.status-' + light).removeClass('status-light-red status-light-orange status-light-green status-light-unknown');
        $('.status-' + light).addClass('status-light-' + color);
        $('.status-' + light + ' .status-light-icon-right').removeClass('fa-triangle-exclamation fa-circle-check fa-circle-question');
        if (color == 'red') {
            $('.status-' + light + ' .status-light-icon-right').addClass('fa-triangle-exclamation');
        } else if (color == 'orange') {
            $('.status-' + light + ' .status-light-icon-right').addClass('fa-triangle-exclamation');
        } else if (color == 'green') {
            $('.status-' + light + ' .status-light-icon-right').addClass('fa-circle-check');
        } else if (color == 'unknown') {
            $('.status-' + light + ' .status-light-icon-right').addClass('fa-circle-question');
        }
    }
}

function setMatricule(el, matricule) {
    $(el).removeClass('matricule-fx matricule-mf matricule-cs matricule-bl');
    $(el).text(matricule);
    if (matricule.length >= 2) {
        $(el).addClass('matricule-' + matricule.substring(0, 2).toLowerCase());
    }
}

function removeMatricule(el) {
    $(el).removeClass('matricule-fx matricule-mf matricule-cs matricule-bl');
    $(el).text('');
}

function setVisible(el, isVisible) {
    setClassIf(el, 'hidden', !isVisible);
}

function setClassIf(el, cls, condition) {
    if (condition) {
        $(el).addClass(cls);
    } else {
        $(el).removeClass(cls);
    }
}

function drawMainClock() {
    let nDots = 60;
    for (let i = 0; i < nDots; i++) {
        let deg = i * 360 / nDots;
        let dot = $('<div class="countdown-dot-container"><div class="countdown-dot" id="countdown-dot-' + i + '"></div></div>');
        dot.css('transform', 'translate(-50%, -50%) rotate(' + deg + 'deg)');
        $('.countdown-dots').append(dot);
    }

    let nDotsLarge = 12;
    for (let i = 0; i < nDotsLarge; i++) {
        let deg = i * 360 / nDotsLarge;
        let dot = $('<div class="countdown-dot-container countdown-dot-container-large"><div class="countdown-dot countdown-dot-on" id="countdown-dot-large-' + i + '"></div></div>');
        dot.css('transform', 'translate(-50%, -50%) rotate(' + deg + 'deg)');
        $('.countdown-dots').append(dot);
    }
}

function updateCountdown(el, lastUpdate, countdown, paused) {
    let currentServerTime = serverTimestamp();
    let ageOfLastUpdate = currentServerTime - lastUpdate;
    let currentCountdown = (paused ? countdown : countdown - ageOfLastUpdate);
    let currentCountdownBak = currentCountdown;
    let positive = (currentCountdown < 0);
    currentCountdown = Math.abs(currentCountdown);
    let countdownHours = Math.floor(currentCountdown / 3600);
    currentCountdown %= 3600;
    let countdownMinutes = Math.floor(currentCountdown / 60);
    currentCountdown %= 60;
    let countdownSeconds = Math.floor(currentCountdown);
    let countdownStr = "T";
    if (positive) {
        countdownStr += "+";
    } else {
        countdownStr += "-";
    }
    if (countdownHours > 0) {
        countdownStr += countdownHours + ':';
    }
    if (countdownMinutes < 10) {
        countdownStr += '0';
    }
    countdownStr += countdownMinutes + ':';
    if (countdownSeconds < 10) {
        countdownStr += '0';
    }
    countdownStr += countdownSeconds;
    $(el).text(countdownStr);
    return currentCountdownBak;
}

function setMainClock(seconds) {
    seconds = Math.abs(Math.floor(seconds) % 60);
    let nDots = 60;
    for (let i = 0; i < nDots; i++) {
        if (i < seconds) {
            $('#countdown-dot-' + i).addClass('countdown-dot-on');
        } else {
            $('#countdown-dot-' + i).removeClass('countdown-dot-on');
        }
    }
}

function updateTimers() {
    updateCurrentTime();

    if (currentState) {
        if (currentState.next_launch) {
            let countdown = updateCountdown('.countdown-value', currentState.next_launch.last_update, currentState.next_launch.countdown, currentState.next_launch.countdown_paused);
            setMainClock(countdown);
        }

        $('.launchpad').each(function(launchpad) {
            let launchpadType = $(launchpad).data('launchpadType');
            let launchpadName = $(launchpad).data('launchpadName');
            let launchpadState = currentState['launchpads_' + launchpadType][launchpadName];
            if (launchpadState != null) {
                updateCountdown('.launchpad-' + launchpadName + '-countdown-value', launchpadState.last_update, launchpadState.countdown, launchpadState.countdown_paused);
            }
        });
    }
}

function connect() {
    console.log("Connecting to the API...");
    if (evtSource != undefined) {
        return;
    }
    evtSource = new EventSource(stateStreamURL);
    evtSource.onopen = (e) => {
        console.log("Connected !");
        setConnectionState(true);
    };
    evtSource.onmessage = (event) => {
        updateState(JSON.parse(event.data));
        setConnectionState(true);
    }
    evtSource.onerror = (e) => {
        console.log("Unable to connect to the API, trying again shortly...");
        setConnectionState(false);
        evtSource.close();
        evtSource = undefined;
        setTimeout(connect, 5000);
    };
}

function setConnectionState(connected) {
    if (connected) {
        $('.overlay-loading').addClass('hidden');
        $('.header').addClass('header-animated');
    } else {
        $('.overlay-loading').removeClass('hidden');
        $('.header').removeClass('header-animated');
    }
}

window.addEventListener("DOMContentLoaded", (event) => {
    drawMainClock();
    computeClockDiff();
    updateTimers();
    update();
    setInterval(updateTimers, 1000);
    connect();
});