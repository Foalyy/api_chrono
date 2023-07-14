import api_chrono

# Configure the API
api_chrono.set_api_url("https://chrono.silica.io")
api_chrono.set_api_key("AeHi8aopuxa9oNgei8ooShai1quahpae")

# Change the colors of the lights
api_chrono.set_zone_mf('green')
api_chrono.set_zone_fx('red')
api_chrono.set_weather('green')
api_chrono.set_zone_safety('red')

# Clear any project listed in Next launch
api_chrono.clear_next_launch()

# Update the project in Next launch
api_chrono.set_next_launch(
    {
        'id': 660,
        'code': 'FX24',
        'name': 'Cobra',
        'club_name': 'LéoFly',
        'rocket_color': 'Noir',
        'parachute_color': 'Rouge',
        'launchpad_name': 'Toutatis',
        'countdown': 180,
        'countdown_paused': False,
    }
)

# Update the list of projects in the clubs tent
api_chrono.set_projects_in_clubs_tent(
    [
        {
            'arrival_time': 1688649923,
            'id': 579,
            'code': 'FX43',
            'name': 'Cyclone',
            'club_name': 'AeroIpsa',
            'rocket_color': 'Green',
            'parachute_color': 'Red/white',
        },
        {
            'arrival_time': 1688649580,
            'id': 680,
            'code': 'MF13',
            'name': 'DART',
            'club_name': 'Planète Sciences',
            'rocket_color': 'Grey',
            'parachute_color': 'Red',
        }
    ]
)

# Update the list of projects in jupiter
api_chrono.set_projects_in_jupiter(
    [
        {
            'arrival_time': 1688649923,
            'id': 620,
            'code': 'FX19',
            'name': 'Minotaur',
            'club_name': 'Polytech Paris-Saclay',
            'rocket_color': 'White',
            'parachute_color': 'Orange',
        },
    ]
)

# Same parameter as set_projects_in_clubs_tent
# api_chrono.set_projects_in_jupiter([ ... ])

# Clear the project on a launchpad
api_chrono.clear_launchpad('fx', 'toutatis')

# Update the project on a launchpad
api_chrono.set_project_on_launchpad('fx', 'toutatis',
    {
        'countdown': 300,
        'countdown_paused': True,
        'waiting_time': 180,
        'arrival_time': 1688648584,
        'id': 579,
        'code': 'FX43',
        'name': 'Cyclone',
        'club_name': 'AeroIpsa',
        'motor_type': 'Pro54',
        'rocket_color': 'Grey',
        'parachute_color': 'Red/white',
    }
)

# Update some fields independently
api_chrono.set_project_on_launchpad('fx', 'toutatis', {
    'rf': 'LoRa',
    'rf_frequency': '868MHz',
})
api_chrono.set_project_on_launchpad('fx', 'toutatis', { 'ddo_ok': True })
api_chrono.set_project_on_launchpad('fx', 'toutatis', {
    'landed': True,
    'flight_result': 'nominal',
})

# Read the current state
# First we send a non-blocking request for the current state,
# then we check state_response() until it returns a non-None value
api_chrono.request_state()
while True:
    state = api_chrono.state_response()
    if state is not None:
        print(state)
        break
    else:
        # Do something else
        pass


# Update the full state
full_state = {
    'zones': {
        'mf': 'red',
        'fx': 'green',
    },
    'next_launch': {
        'id': 660,
        'code': 'FX24',
        'name': 'Cobra',
        'club_name': 'LéoFly',
        'rocket_color': 'Noir',
        'parachute_color': 'Rouge',
        'launchpad_name': 'Toutatis',
        'countdown': 180,
        'countdown_paused': False,
    },
    # 'next_launch': None,
    'clubs_tent': [
        {
            'arrival_time': 1688649923,
            'id': 579,
            'code': 'FX43',
            'name': 'Cyclone',
            'club_name': 'AeroIpsa',
            'rocket_color': 'Vert',
            'parachute_color': 'Rouge/blanc',
        },
        {
            'arrival_time': 1688649580,
            'id': 680,
            'code': 'MF13',
            'name': 'DART',
            'club_name': 'Planète Sciences',
            'rocket_color': 'Gris',
            'parachute_color': 'Rouge',
        }
    ],
    'launchpads_fx': {
        'toutatis': {
            'countdown': 300,
            'countdown_paused': True,
            'waiting_time': 180,
            'arrival_time': 1688648584,
            'id': 579,
            'code': 'FX43',
            'name': 'Cyclone',
            'club_name': 'AeroIpsa',
            'rocket_color': 'Grey',
            'parachute_color': 'Red/white',
            'special_project': False,
            'upper_ailerons': False,
            'two_stage': False,
            'roll_control': False,
            'supersonic': False,
            'rf': 'LoRa',
            'rf_onboard': True,
            'rf_check': False,
            'rf_on': False,
            'rf_frequency': '868MHz',
            'launchpad_set': False,
            'motor_type': '',
            'motor_destocked': False,
            'motor_inserted': False,
            'motor_wired': False,
            'pyro_ok': False,
            'club_ok': False,
            'ddo_ok': False,
            'rso_ok': False,
            'landed': False,
            'flight_result': 'unknown',
        }
    }
}
#api_chrono.post(full_state)


api_chrono.close()