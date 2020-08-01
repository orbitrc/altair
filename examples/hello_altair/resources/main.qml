import QtQuick.Window 2.12

import HelloWorld 1.0

Window {
  id: root

  visible: true

  width: 300
  height: 300

  HelloWorld {
    anchors.fill: parent
  }
}
