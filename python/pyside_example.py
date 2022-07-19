from promethean import LevelGenerator, Level, Options, Tile, LevelStatistics

from PySide6 import QtWidgets, QtGui, QtCore
import math

SETTINGS_SIZE: int = 256
MAX_SIZE = 2048
MAX_SEED = 4096
BG_COLOR = (125, 125, 125, 255)
TILE_COLOR = (196, 196, 196, 128)
WALL_COLOR = (196, 64, 64, 128)
CENTER_COLOR = (32, 196, 32, 64)


class PointTransformer():
    def __init__(self, min_corner, max_corner, shift, aspect, width, height):
        self._min_corner = min_corner
        self._max_corner = max_corner
        self._shift = shift
        self._aspect = aspect
        self._width = width
        self._height = height

        self._x_coeff = self._height * self._aspect if self._aspect <= self._width / self._height else self._width
        self._y_coeff = self._height if self._aspect <= self._width / self._height else self._width / self._aspect

    def transform(self, point):
        return ((point[0] - self._min_corner[0]) * self._x_coeff / (self._max_corner[0] - self._min_corner[0]) + self._shift[0],
                (point[1] - self._min_corner[1]) * self._y_coeff / (self._max_corner[1] - self._min_corner[1]) + self._shift[1])

    def transform_inverse(self, point):
        return ((point[0] - self._shift[0]) * (self._max_corner[0] - self._min_corner[0]) / self._x_coeff + self._min_corner[0],
                (point[1] - self._shift[1]) * (self._max_corner[1] - self._min_corner[1]) / self._y_coeff + self._min_corner[1])


class SettingsWidget(QtWidgets.QWidget):
    def __init__(self, graphic_widget):
        super(SettingsWidget, self).__init__()
        self.setMinimumWidth(SETTINGS_SIZE)
        self.setMaximumWidth(SETTINGS_SIZE)

        self._graphic_widget = graphic_widget

        self._options_level_width = QtWidgets.QSpinBox()
        self._options_level_width.setValue(64)
        self._options_level_width.setMinimum(4)
        self._options_level_width.setMaximum(MAX_SIZE)
        self._options_level_width.valueChanged.connect(self.generate)

        self._options_level_height = QtWidgets.QSpinBox()
        self._options_level_height.setValue(64)
        self._options_level_height.setMinimum(4)
        self._options_level_height.setMaximum(MAX_SIZE)
        self._options_level_height.valueChanged.connect(self.generate)

        self._options_min_room_width = QtWidgets.QSpinBox()
        self._options_min_room_width.setValue(5)
        self._options_min_room_width.setMinimum(1)
        self._options_min_room_width.setMaximum(MAX_SIZE)
        self._options_min_room_width.valueChanged.connect(self.generate)

        self._options_max_room_width = QtWidgets.QSpinBox()
        self._options_max_room_width.setValue(7)
        self._options_max_room_width.setMinimum(1)
        self._options_max_room_width.setMaximum(MAX_SIZE)
        self._options_max_room_width.valueChanged.connect(self.generate)

        self._options_min_room_height = QtWidgets.QSpinBox()
        self._options_min_room_height.setValue(5)
        self._options_min_room_height.setMinimum(1)
        self._options_min_room_height.setMaximum(MAX_SIZE)
        self._options_min_room_height.valueChanged.connect(self.generate)

        self._options_max_room_height = QtWidgets.QSpinBox()
        self._options_max_room_height.setValue(7)
        self._options_max_room_height.setMinimum(1)
        self._options_max_room_height.setMaximum(MAX_SIZE)
        self._options_max_room_height.valueChanged.connect(self.generate)

        self._options_number_of_rooms = QtWidgets.QSpinBox()
        self._options_number_of_rooms.setValue(45)
        self._options_number_of_rooms.setMinimum(1)
        self._options_number_of_rooms.setMaximum(MAX_SIZE)
        self._options_number_of_rooms.valueChanged.connect(self.generate)

        self._options_random_seed = QtWidgets.QSpinBox()
        self._options_random_seed.setValue(1)
        self._options_random_seed.setMinimum(0)
        self._options_random_seed.setMaximum(MAX_SEED)

        self._options_border = QtWidgets.QSpinBox()
        self._options_border.setValue(1)
        self._options_border.setMinimum(1)
        self._options_border.setMaximum(MAX_SIZE)
        self._options_border.valueChanged.connect(self.generate)

        self._options_room_border = QtWidgets.QSpinBox()
        self._options_room_border.setValue(1)
        self._options_room_border.setMinimum(1)
        self._options_room_border.setMaximum(MAX_SIZE)
        self._options_room_border.valueChanged.connect(self.generate)

        self._options_overlap_rooms = QtWidgets.QCheckBox()
        self._options_overlap_rooms.setChecked(False)
        self._options_overlap_rooms.stateChanged.connect(self.generate)

        self._generate_button = QtWidgets.QPushButton("Generate")
        self._generate_button.clicked.connect(self.generate)

        level_size_box = QtWidgets.QGroupBox("Level Size")
        room_size_box = QtWidgets.QGroupBox("Room Size")
        generate_box = QtWidgets.QGroupBox("Generate")

        self._level_size_layout = QtWidgets.QFormLayout()
        self._level_size_layout.addRow("&Level Width:", self._options_level_width)
        self._level_size_layout.addRow("&Level Height:", self._options_level_height)
        self._level_size_layout.addRow("&Level Border:", self._options_border)

        self._room_size_layout = QtWidgets.QFormLayout()
        self._room_size_layout.addRow("&Min Room Width:", self._options_min_room_width)
        self._room_size_layout.addRow("&Max Room Width:", self._options_max_room_width)
        self._room_size_layout.addRow("&Min Room Height:", self._options_min_room_height)
        self._room_size_layout.addRow("&Max Room Height:", self._options_max_room_height)
        self._room_size_layout.addRow("&Room Border:", self._options_room_border)

        self._generate_layout = QtWidgets.QFormLayout()
        self._generate_layout.addRow("&Rooms Count:", self._options_number_of_rooms)
        self._generate_layout.addRow("Seed:", self._options_random_seed)
        self._generate_layout.addRow("&Overlap Rooms:", self._options_overlap_rooms)

        level_size_box.setLayout(self._level_size_layout)
        room_size_box.setLayout(self._room_size_layout)
        generate_box.setLayout(self._generate_layout)

        self._layout = QtWidgets.QVBoxLayout()
        self._layout.addWidget(level_size_box)
        self._layout.addWidget(room_size_box)
        self._layout.addWidget(generate_box)
        self._layout.addWidget(self._generate_button)
        self._layout.addStretch()

        self.setLayout(self._layout)

    def generate(self):
        self._options_random_seed.setValue(self._options_random_seed.value() + 1)
        # create generator options
        options: Options = Options()
        options.level_width = self._options_level_width.value()
        options.level_height = self._options_level_height.value()
        options.min_room_width = self._options_min_room_width.value()
        options.max_room_width = self._options_max_room_width.value()
        options.min_room_height = self._options_min_room_height.value()
        options.max_room_height = self._options_max_room_height.value()
        options.number_of_rooms = self._options_number_of_rooms.value()
        options.random_seed = self._options_random_seed.value()
        options.border = self._options_border.value()
        options.room_border = self._options_room_border.value()
        options.overlap_rooms = self._options_overlap_rooms.isChecked()
        self._graphic_widget.generate(options)


class GraphicWidget(QtWidgets.QWidget):
    def __init__(self, main):
        super(GraphicWidget, self).__init__()
        self._level = None
        self._main = main
        self._tile_pen = QtGui.QPen(QtGui.QColor(*TILE_COLOR))
        self._tile_brush = QtGui.QBrush(QtGui.QColor(*TILE_COLOR))

        self._wall_pen = QtGui.QPen(QtGui.QColor(*WALL_COLOR))
        self._wall_brush = QtGui.QBrush(QtGui.QColor(*WALL_COLOR))

        self._center_pen = QtGui.QPen(QtGui.QColor(*CENTER_COLOR))
        self._center_brush = QtGui.QBrush(QtGui.QColor(*CENTER_COLOR))

    def paintEvent(self, event):
        canvas_width = self.size().width()
        canvas_height = self.size().height()
        painter = QtGui.QPainter(self)
        painter.setRenderHint(QtGui.QPainter.Antialiasing)
        painter.fillRect(0, 0, canvas_width, canvas_height, QtGui.QColor(*BG_COLOR))

        if self._level is not None:
            tiles: List[List[Tile]] = self._level.render()
            height_count = self._level.get_height()
            width_count = self._level.get_width()
            
            graph_aspect = width_count / height_count
            if graph_aspect > canvas_width / canvas_height:
                x_shift = 0
                y_shift = (canvas_height - canvas_width / graph_aspect) / 2
            else:
                y_shift = 0
                x_shift = (canvas_width - canvas_height * graph_aspect) / 2

            self._tfm = PointTransformer((0.0, 0.0),
                                         (width_count, height_count),
                                         (x_shift, y_shift),
                                         graph_aspect,
                                         canvas_width, canvas_height)
            for row in range(height_count):
                for column in range(width_count):
                    t = tiles[row][column]
                    if t != Tile.Empty:
                        if t == Tile.Floor:
                            painter.setPen(self._tile_pen)
                            painter.setBrush(self._tile_brush)
                        else:
                            painter.setPen(self._wall_pen)
                            painter.setBrush(self._wall_brush)
                        point_x, point_y = self._tfm.transform((column, row))
                        size_x, size_y = self._tfm.transform((column + 1, row + 1))
                        rectangle = QtCore.QRectF(point_x, point_y, size_x - point_x, size_y - point_y)
                        painter.drawRect(rectangle)
            # draw room center
            centers = self._level.get_statistics().room_centers
            painter.setPen(self._center_pen)
            painter.setBrush(self._center_brush)
            for c in centers:
                point_x, point_y = self._tfm.transform((c._y, c._x))
                painter.drawEllipse(QtCore.QPoint(point_x, point_y), 3, 3)


    def generate(self, options: Options):
        generator = LevelGenerator(options)
        self._level = generator.generate()
        self._main.set_status(self._level.get_statistics())
        self.repaint()


class PrometheanMainWidget(QtWidgets.QWidget):
    def __init__(self, main_window):
        super(PrometheanMainWidget, self).__init__()
        self._layout = QtWidgets.QHBoxLayout(self)
        self._graphics_widget = GraphicWidget(main_window)
        self._settings_widget = SettingsWidget(self._graphics_widget)
        self._layout.addWidget(self._graphics_widget)
        self._layout.addWidget(self._settings_widget)


class PrometheanApp(QtWidgets.QMainWindow):
    def __init__(self):
        super(PrometheanApp, self).__init__()
        self._main = PrometheanMainWidget(self)
        self.setCentralWidget(self._main)
        self.setWindowTitle("Promethean")
        self._status = self.statusBar()

    def set_status(self, level_statistics: LevelStatistics):
        if level_statistics.init:
            self._status.showMessage("Rooms: " + str(level_statistics.rooms_count) + (". Some rooms can be isolated" if level_statistics.all_corridors is False else ""))
        else:
            self._status.showMessage("")


if __name__ == "__main__":
    app = QtWidgets.QApplication()
    main = PrometheanApp()
    main.resize(800, 600)
    main.show()
    app.exec()