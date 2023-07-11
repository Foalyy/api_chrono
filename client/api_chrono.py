import threading, queue, requests

api_url = ""
api_key = ""
timeout = 1

main_to_bg_queue = queue.Queue()
bg_to_main_queue = queue.Queue()

def _bg_thread_worker():
    while True:
        message, payload = main_to_bg_queue.get()
        if message == 'get':
            resp = requests.get(f'{api_url}/state', timeout=timeout).json()
            bg_to_main_queue.put(resp)
        elif message == 'post':
            requests.post(f'{api_url}/state', headers={'apikey': api_key}, json=payload, timeout=timeout)
        elif message == 'reset':
            requests.post(f'{api_url}/reset', headers={'apikey': api_key}, timeout=timeout)
        main_to_bg_queue.task_done()

bg_thread = threading.Thread(target=_bg_thread_worker, daemon=True)
bg_thread.start()

def set_api_url(url):
    global api_url
    api_url = url

def set_api_key(key):
    global api_key
    api_key = key

def post(state):
    main_to_bg_queue.put(('post', state))

def request_state():
    main_to_bg_queue.put(('get', None))

def state_response():
    state = None
    while not bg_to_main_queue.empty():
        try:
            state = bg_to_main_queue.get(block=False)
        except queue.Empty:
            break
    return state

def reset_state():
    main_to_bg_queue.put(('reset', None))


def set_zone_mf(color):
    post({
        'zones': {
            'mf': color
        }
    })

def set_zone_fx(color):
    post({
        'zones': {
            'fx': color
        }
    })

def set_next_launch(next_launch):
    post({
        'next_launch': next_launch
    })

def clear_next_launch():
    post({
        'next_launch': None
    })

def set_projects_in_clubs_tent(projects):
    post({
        'clubs_tent': projects
    })

def set_projects_in_jupiter(projects):
    post({
        'jupiter': projects
    })

def clear_launchpad(zone, launchpad):
    post({
        f'launchpads_{zone.lower()}': {
            launchpad: None
        }
    })

def set_project_on_launchpad(zone, launchpad, project):
    post({
        f'launchpads_{zone.lower()}': {
            launchpad: project
        }
    })
    