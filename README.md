= ha-notifier

A simple utility to send an mqtt notification to home assistant which can then trigger a notification.

== Usage
=== Home Assitant
You'll need an MQTT broker such as Mosquitto installed and you'll need a login for your notification tool.

1. **Settings**->**Automations & Scenes**->**Create automation**
2. **Add trigger**
  1. **By Type**->**MQTT**->**MQTT message received**
  2. **Topic** = `notification/target` where `target` is the subtopic, often your name or other identifier.
  3. **Payload** (checked) = a word or sentance that will trigger a specific behavior. Optional.
3. Optional **Add condition** - This will only send the notification if you are in a specific zone
  1. **Add target** -> Entities -> Mobile app device you carry
  2. **Zone** -> The zone you want to be in (for example **Home**)
4. **Add Action**
  1. **By Type**->**Notifications**->**Send a notification message**
  2. **Add target** = The device you want to notify (phone, iPad, etc.)
  3. **Message** = Whatever message you want
  4. Optional **Title** = A short title of your message is long
5. Save the notification and test it.

== CLI
1. set env variables (Optional, can use cli args)
  1. **HA_NOTIFIER_URL** The mqtt url. Don't need to set username or password. **client_id** is optional and will default to "ha-notification"
  2. **HA_NOTIFIER_USERNAME** The username you set when creating an mqtt broker login.
  3. **HA_NOTIFIER_PASSWORD** The password you set when creating an mqtt broker login.
2. `ha-notifier <subtopic> <message...>` where `subtopic` is the `target` you se in 2.2 above and message is the `payload` you set in 2.3. If you didn't set a payload this can be anything.

`ha-notifier --help` will show you how to pass url, username, and password on the command line.

== Example
Set the notification `target` to "mobile" and `payload` to "claude".
Set the `title` to "Claude waiting" and the `message` to "Claude is waiting for input from you".

Add this to `.claude/settings.json` wither in your project or in your home directory.
```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "ha-notify mobile claude"
          }
        ]
      }
    ]
  }
}
```

Restart claude code and you should get a mobile notification whenever you need to check your session.
