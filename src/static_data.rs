pub static main_cpp: &str = "
#include \"Engine.hpp\"
#include <chrono>
#include <glad/glad.h>
#include <iostream>
#include <memory>
#include <thread>

// #include \"particle.hpp\"

int main(int argc, char *argv[]) {

  std::cout << \"Engine starting here ! \" << argv[0] << std::endl;
  {
    // create an instance of Engine
    IND_PATH(argv[0]);
    std::shared_ptr<SHM::Engine> engine =
        SHM::Engine::startEngine(\"#PROJECT_NAME#\", SHM::OPENGL, argv[0]);
    // main render loop
    engine->MainRenderLoop();
  }

  std::cout << \"Goodbye\" << std::endl;
  return 0;
}
";

pub static cmake_file_content: &str = "
cmake_minimum_required(VERSION 3.14)

project(#PROJECT_NAME#)
set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
file(GLOB Game_SRC CONFIGURE_DEPENDS \"sources/*.cpp\" \"sources/*.c\")

add_executable(
	#PROJECT_NAME# 
    ${Game_SRC}
)

target_include_directories(#PROJECT_NAME# PUBLIC includes/ Engine/include/ Engine/openGLRenderer/ shm-physics/include/cyclon/ cyclone-physics/include/cyclone)
target_compile_options(#PROJECT_NAME# PUBLIC -g)

if (WIN32)
	target_link_libraries(#PROJECT_NAME# \"F:/project/SHM/Engine/include/GLFW/glfw3.lib\")
	target_link_libraries(#PROJECT_NAME# \"F:/project/SHM/Engine/include/assimp/assimp-vc142-mtd.lib\")

elseif(UNIX)
	target_link_libraries(#PROJECT_NAME# ${CMAKE_DL_LIBS})
	target_link_libraries(#PROJECT_NAME# Threads::Threads)
	target_link_libraries(#PROJECT_NAME# glfw3)
	target_link_libraries(#PROJECT_NAME# assimp)
endif()
target_link_libraries(#PROJECT_NAME# shm-engine)

# copy the asset folder to generated bin file
# so the asset folder can be used in the game
# and the path can be relative
add_custom_command(
	TARGET #PROJECT_NAME# 
	POST_BUILD
	COMMAND ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/assets ${CMAKE_BINARY_DIR}/bin/assets
)
message(${CMAKE_CURRENT_SOURCE_DIR}/assets)

";

pub static load_model_header: &str = "
#ifndef PLAYGROUND_MODEL_ACTOR_H
#define PLAYGROUND_MODEL_ACTOR_H

#include \"Actor.hpp\"
#include \"InputHandler.hpp\"
// #include \"birdPhysic.hpp\"

class GameModel : public SHM::Actor {
public:
  GameModel();
  virtual ~GameModel();

  void setUpModel() override;
  void eachFrame() override;
  void jump() override;
};

class JumpAction : public SHM::Command {
  void execute(SHM::BaseActor *actor) override;
};

class WAction : public SHM::Command {
  void execute(SHM::BaseActor *actor) override;
};

#endif // PLAYGROUND_MODEL_ACTOR_H
";

pub static load_model_source: &str = "
#include \"firstmodel.hpp\"

GameModel::GameModel() {
  SHM::Engine::GetEngine()->getRenderer()->m_actors.push_back(this);
  SHM::Engine::GetEngine()->setMovingCharacter(this);
  SHM::Command *jump_action = new JumpAction{};
  SHM::Command *w_action = new WAction{};
  // TODO free this two action pointer on destruction
  SHM::Engine::GetEngine()->setActionToKey(SHM::KEY::KEYBOARD_SPACE,
                                           jump_action);
  SHM::Engine::GetEngine()->setActionToKey(SHM::KEY::KEYBOARD_W, w_action);

  std::cout << \"BIRD Actor created!\" << std::endl;
}
GameModel::~GameModel() { std::cout << \"BIRD Actor destroyed!\" << std::endl; }

void GameModel::setUpModel() {
  this->model->setPosition({-3.0f, 5.7f, 0.0f});
  this->model->setScale({0.5f, 0.5f, 0.5f});
  // this->model->setRotation({0.0f, 1.0f, 0.0f});
  this->model->setRotation({0.0f, -1.0f, 0.0f});
  SHM::Engine::GetEngine()->getCamera()->attachTo(
      this, (uint8_t)SHM::Camera::ATTACH::X);
  //   this->m_physic_component =
  //       new BirdPhysic{this->model->getPosition(), {0.1f, 0.0f}, true};
}
void GameModel::eachFrame() {
  //   if (this->m_physic_component->isAwake)
  //     this->model->setPosition({this->model->getPosition()->x + 0.05, 5.7,
  //     0});
}

void JumpAction::execute(SHM::BaseActor *actor) {
  if (actor)
    actor->jump();
}

void WAction::execute(SHM::BaseActor *actor) {
  if (actor) {
    std::cout << \"W key pressed!\" << std::endl;
  }
}

void GameModel::jump() {
  std::cout << \"BIRD ACTOR IS CALLING JUMP!\" << std::endl;
}
";

pub static shm_minimal_setting: &str = "
#include \"Engine.hpp\"
#include \"Light.hpp\"
#include \"buffers.hpp\"
#include \"firstmodel.hpp\"
#include \"glm/gtx/string_cast.hpp\"

#define GET_MODEL(x) Engine::getRenderer()->getModelByIndex(x)
#define REGISTER_MODEL(x) Engine::getRenderer()->registerModel(x)

namespace SHM {

// set out of render-loop config and command
void Engine::outLoop(GLFWwindow *window) {
  Engine::m_camera->m_position = glm::vec3{2.0f, 4.7f, 11.0f};
  static GameModel pipe{};
  pipe.setShader(\"/assets/second/model_loading.vs\",
                 \"/assets/second/model_loading.fs\");
  pipe.loadModel(
      std::string{Engine::cwd + \"/assets/second/texturedpipe.obj\"}.c_str());

  pipe.setUpModel();
}

// set in loop config and command
void Engine::inLoop() {
  Engine::m_camera->m_front = glm::vec3{0.0f, 0.0f, -1.0f};
}

} // namespace SHM

";
