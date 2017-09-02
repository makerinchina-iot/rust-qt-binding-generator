import QtQuick 2.6
import QtQuick.Controls 2.0
import QtQuick.Layouts 1.3

ApplicationWindow {
    id: application
    property string initialTab: "style"
    property int qtquickIndex: 0
    visible: true
    footer: StyleSwitcher {
        anchors.fill: parent
    }
    contentItem {
        TabBar {
            id: bar
            width: parent.width
            TabButton {
                text: "object"
            }
            TabButton {
                text: "list"
            }
            TabButton {
                text: "tree"
            }
        }
        StackLayout {
            width: parent.width
            anchors.top: bar.bottom
            anchors.bottom: parent.bottom
            currentIndex: bar.currentIndex
            ColumnLayout {
                ComboBox {
                    currentIndex: qtquickIndex
                    model: styles
                    textRole: "display"
                    onCurrentIndexChanged: {
                        if (currentIndex != qtquickIndex) {
                            widgets.currentIndex = currentIndex
                            application.close()
                        }
                    }
                }
                Image {
                    source: "../logo.svg"
                }
            }
            RowLayout {
                TextField {
                    id: fibonacciInput
                    placeholderText: "Your number"
                    validator: IntValidator {
                        bottom: 0
                        top: 100
                    }
                    Component.onCompleted: {
                        text = demo.fibonacci.input
                    }
                    onTextChanged: {
                        demo.fibonacci.input = parseInt(text, 10)
                    }
                }
                Text {
                    text: "The Fibonacci number: " + demo.fibonacci.result
                }
            }
            ListView {
                id: listView
                model: demo.fibonacciList
                delegate: Row {
                    Text {
                        text: result
                    }
                }
            }
            Text {
                id: treeView
                text: "No TreeView in QtQuick Controls 2"
            }
        }
    }
}
